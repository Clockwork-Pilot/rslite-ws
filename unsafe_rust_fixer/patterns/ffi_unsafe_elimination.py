"""Plugin: FFI Unsafe Elimination

Full AST + data-flow analysis to eliminate unnecessary unsafe in FFI/C-binding code.

Sub-patterns handled:

  A. cast_binding_to_safe_ref
     ─────────────────────────
     Finds:
       let [mut] p: *mut SomeType = raw_ptr as *mut SomeType;
       ... (*p).field1 ... (*p).field2 ...   (only field-deref uses)

     Fixes:
       let p = &[mut] *(raw_ptr as *mut SomeType);   // safe ref binding
       ... p.field1 ... p.field2 ...                  // safe field access

     Conditions:
       - The binding RHS is a cast expression (expr as *mut/const T)
       - The variable is only used via (*var).field (no raw passes to fn, no reassignment)
       - At least 1 dereference use

  B. nested_deref_hoist
     ───────────────────
     Finds repeated occurrences (≥ 2) of:
       (*(*ptr).inner_ptr).subfield

     Fixes:
       let __inner_ptr_ref = &[mut] *(*ptr).inner_ptr;   // hoisted binding
       __inner_ptr_ref.subfield                           // safe access

     Conditions:
       - The inner pointer expression (*ptr).inner_ptr appears ≥ 2 times
       - Not reassigned between uses
       - Correct mutability detection (LHS vs RHS)

Data-flow analysis per binding:
  - Track all defs (declarations, assignments)
  - Track all uses (field access, function args, raw deref, address-of)
  - Classify each use type to determine safety of transformation
  - Detect pointer reassignment to block unsafe transformations

Works exclusively in unsafe extern "C" functions (FFI boundaries).
"""

import re
import hashlib
import json
import os
from typing import List, Tuple, Optional, Any, Dict, Set
from dataclasses import dataclass
from collections import defaultdict

from .base import UnsafePatternPlugin

_CACHE_DIR = "/tmp/ffi_unsafe_elimination_cache"

# ── Regex pre-filters ────────────────────────────────────────────────────────

# Detect any (*VAR).field pattern (needed for both sub-patterns)
DEREF_FIELD_RE = re.compile(r'\(\*(\w+)\)\s*\.\s*\w+')

# Detect cast binding: let [mut] VAR: *mut/const T = EXPR as *mut/const
CAST_BINDING_RE = re.compile(
    r'\blet\s+(?:mut\s+)?(\w+)\s*(?::\s*\*(?:const|mut)[^=]+)?\s*=\s*.+\s+as\s+\*(?:const|mut)'
)

# Detect nested double-deref: (*(*VAR).FIELD).SUBFIELD
NESTED_DEREF_RE = re.compile(
    r'\(\*\(\*(\w+)\)\s*\.\s*(\w+)\)\s*\.\s*(\w+)'
)


@dataclass
class CastBinding:
    """A raw pointer binding created via cast."""
    var_name: str
    is_mut: bool          # Whether *mut (True) or *const (False)
    decl_node: Any        # The let_declaration AST node
    rhs_expr: str         # The RHS expression (e.g., "pCursor as *mut Fts3auxCursor")
    inner_expr: str       # The expression being cast (e.g., "pCursor")
    cast_type: str        # The type being cast to (e.g., "*mut Fts3auxCursor")


@dataclass
class BindingUsages:
    """All usages of a named binding, categorised for safety analysis."""
    field_derefs: List[Any]   # (*var).field — safe to convert
    raw_derefs: List[Any]     # *var (not field access) — must keep unsafe
    fn_args: List[Any]        # var passed as argument to function — can't convert
    reassignments: List[Any]  # var = new_value — invalidates safe ref
    compound_assigns: List[Any]  # var.field += value — needs &mut
    method_calls: List[Any]   # (*var).method(args) — needs &mut if binding is *mut

    @property
    def total_uses(self) -> int:
        return (len(self.field_derefs) + len(self.raw_derefs) +
                len(self.fn_args) + len(self.reassignments) +
                len(self.compound_assigns) + len(self.method_calls))

    @property
    def only_field_derefs(self) -> bool:
        """True iff the variable is only used via field/method access — safe to convert."""
        return (self.raw_derefs == [] and
                self.fn_args == [] and
                self.reassignments == [])

    @property
    def needs_mut_ref(self) -> bool:
        """True if the safe reference needs to be &mut."""
        return len(self.compound_assigns) > 0 or any(
            self._is_lhs(fe) for fe in self.field_derefs
        )

    @staticmethod
    def _is_lhs(node: Any) -> bool:
        """Check if node appears on the LHS of an assignment."""
        cur = node
        while cur is not None:
            parent = cur.parent
            if parent is None:
                break
            if parent.type in ('assignment_expression', 'compound_assignment_expr'):
                left = parent.child_by_field_name('left')
                if left is not None:
                    if left.start_byte <= node.start_byte and node.end_byte <= left.end_byte:
                        return True
            if parent.type == 'block':
                break
            cur = parent
        return False


@dataclass
class NestedDerefGroup:
    """A group of nested double-dereferences sharing the same inner pointer."""
    outer_var: str         # The outer pointer variable (e.g., "p")
    inner_field: str       # The field of the outer pointer (e.g., "pFts3Tab")
    inner_expr: str        # Full inner expression (e.g., "(*p).pFts3Tab")
    uses: List[Any]        # All field_expression nodes matching (*(*p).pFts3Tab).sub
    block: Any             # The enclosing block
    needs_mut: bool = False


class FFIUnsafeEliminationPlugin(UnsafePatternPlugin):
    """Eliminate unnecessary unsafe in FFI/C-binding code."""

    @property
    def name(self) -> str:
        return "ffi_unsafe_elimination"

    @property
    def description(self) -> str:
        return (
            "Eliminate unnecessary unsafe in FFI/C-binding code: convert cast pointer "
            "bindings to safe references, hoist nested double-deref patterns"
        )

    @property
    def priority(self) -> int:
        return 15  # Higher than raw_ptr_deref_field_chain (14)

    # ── AST helpers ─────────────────────────────────────────────────────────

    def _is_direct_deref_field(self, field_expr: Any, code: str) -> Optional[str]:
        """Return the pointer variable name if field_expr is (*VAR).field, else None."""
        value = field_expr.child_by_field_name('value')
        if value is None or value.type != 'parenthesized_expression':
            return None
        for child in value.children:
            if child.type == 'unary_expression':
                children = child.children
                if (len(children) >= 2 and children[0].type == '*' and
                        children[1].type == 'identifier'):
                    return self.node_text(children[1], code)
        return None

    def _is_nested_deref_field(self, field_expr: Any, code: str) -> Optional[Tuple[str, str]]:
        """
        Return (outer_var, inner_field) if field_expr is (*(*VAR).FIELD).SUBFIELD.
        The outer_var is VAR and inner_field is FIELD.
        """
        value = field_expr.child_by_field_name('value')
        if value is None or value.type != 'parenthesized_expression':
            return None
        for child in value.children:
            if child.type != 'unary_expression':
                continue
            children = child.children
            if len(children) < 2 or children[0].type != '*':
                continue
            # children[1] should be a field_expression: (*VAR).FIELD
            inner_field_expr = children[1]
            if inner_field_expr.type != 'field_expression':
                continue
            # inner_field_expr.value must be (*VAR)
            outer_var = self._is_direct_deref_field(inner_field_expr, code)
            if outer_var is None:
                continue
            inner_field_node = inner_field_expr.child_by_field_name('field')
            if inner_field_node is None:
                continue
            inner_field = self.node_text(inner_field_node, code)
            return (outer_var, inner_field)
        return None

    def _find_function_scope(self, node: Any) -> Optional[Any]:
        """Find the enclosing function_item for node."""
        cur = node
        while cur is not None:
            if cur.type == 'function_item':
                return cur
            cur = cur.parent
        return None

    def _is_extern_c_unsafe_function(self, node: Any, code: str) -> bool:
        """
        True if node is inside an unsafe extern "C" function specifically.
        Regular unsafe fn (without extern "C") is excluded — those are not FFI
        boundaries and have different safety properties.
        """
        func = self._find_function_scope(node)
        if func is None:
            return False
        # Get just the function header up to the opening brace
        func_text = self.node_text(func, code)
        brace = func_text.find('{')
        header = func_text[:brace] if brace != -1 else func_text[:300]
        # Must have both 'unsafe' and 'extern' with "C" ABI
        has_unsafe = 'unsafe' in header
        has_extern_c = ('extern "C"' in header or "extern 'C'" in header)
        return has_unsafe and has_extern_c

    def _find_cast_binding(self, block: Any, var_name: str, code: str
                           ) -> Optional[CastBinding]:
        """
        Find the let declaration in block for var_name that is a cast-binding.
        Pattern: let [mut] var_name: *[mut|const] T = EXPR as *[mut|const] T;
        """
        for child in block.children:
            if child.type != 'let_declaration':
                continue

            # Get pattern node (variable name, possibly prefixed with mut)
            pattern = child.child_by_field_name('pattern')
            if pattern is None:
                continue

            pat_text = self.node_text(pattern, code).strip()
            # Strip leading 'mut ' from pattern text
            bare_pat = pat_text.lstrip('mut').strip()

            if bare_pat != var_name:
                continue

            # Get type annotation
            type_node = child.child_by_field_name('type')
            if type_node is None:
                continue
            type_text = self.node_text(type_node, code).strip()
            if not ('*mut' in type_text or '*const' in type_text):
                continue

            # Get the value (RHS)
            value_node = child.child_by_field_name('value')
            if value_node is None:
                continue

            rhs_text = self.node_text(value_node, code).strip()

            # RHS must be a cast expression: EXPR as *mut/const T
            if not re.search(r'\bas\s+\*(?:const|mut)\s+\S', rhs_text):
                continue

            # Extract inner expression and cast type
            cast_match = re.search(
                r'^(.*?)\s+as\s+(\*(?:const|mut)\s+\S+)\s*$',
                rhs_text, re.DOTALL
            )
            if cast_match is None:
                continue

            inner_expr = cast_match.group(1).strip()
            cast_type = cast_match.group(2).strip()
            is_mut = '*mut' in cast_type

            return CastBinding(
                var_name=var_name,
                is_mut=is_mut,
                decl_node=child,
                rhs_expr=rhs_text,
                inner_expr=inner_expr,
                cast_type=cast_type,
            )

        return None

    def _collect_usages(self, block: Any, var_name: str, code: str) -> BindingUsages:
        """
        Full data-flow analysis: find all uses of var_name in block.
        Classifies each use into:
          - field_derefs: (*var).field  (safe to convert)
          - raw_derefs: *var as standalone (cannot safely convert)
          - fn_args: var used as fn arg (type changes are visible to callee)
          - reassignments: var = ...  (invalidates safe ref)
          - compound_assigns: (*var).field += ... (needs &mut)
        """
        field_derefs: List[Any] = []
        raw_derefs: List[Any] = []
        fn_args: List[Any] = []
        reassignments: List[Any] = []
        compound_assigns: List[Any] = []
        method_calls: List[Any] = []

        # Collect field dereferences: (*var).field and (*var).method(args)
        for fe in self.find_nodes(block, 'field_expression'):
            ptr = self._is_direct_deref_field(fe, code)
            if ptr == var_name:
                parent = fe.parent
                # Method call: (*var).method(args) — the field_expr is the function
                if parent and parent.type == 'call_expression':
                    func_child = parent.child_by_field_name('function')
                    if func_child is not None and func_child.start_byte == fe.start_byte:
                        method_calls.append(fe)
                        continue
                # Compound assignment: (*var).field += ...
                if parent and parent.type == 'compound_assignment_expr':
                    left = parent.child_by_field_name('left')
                    if left is not None and left.start_byte == fe.start_byte:
                        compound_assigns.append(fe)
                        continue
                field_derefs.append(fe)

        # Collect raw dereferences: *var (not via field expression)
        for ue in self.find_nodes(block, 'unary_expression'):
            ue_children = ue.children
            if (len(ue_children) < 2 or
                    ue_children[0].type != '*' or
                    ue_children[1].type != 'identifier'):
                continue
            if self.node_text(ue_children[1], code) != var_name:
                continue
            # Skip if this is inside (*var).field (already counted)
            parent = ue.parent
            if (parent and parent.type == 'parenthesized_expression' and
                    parent.parent and parent.parent.type == 'field_expression'):
                continue
            raw_derefs.append(ue)

        # Collect function argument uses: any call where var_name appears as arg
        for call in self.find_nodes(block, 'call_expression'):
            args_node = call.child_by_field_name('arguments')
            if args_node is None:
                continue
            # Check for var_name appearing as an identifier in args
            for ident in self.find_nodes(args_node, 'identifier'):
                if self.node_text(ident, code) == var_name:
                    fn_args.append(call)
                    break

        # Collect method call uses: var.method(...) — raw pointer methods like
        # .is_null(), .offset(), .add(), .sub(), .cast(), .read(), .write()
        # cannot be called on safe references, so these mark the var unsafe to convert.
        for mcall in self.find_nodes(block, 'call_expression'):
            func_node = mcall.child_by_field_name('function')
            if func_node is None or func_node.type != 'field_expression':
                continue
            receiver = func_node.child_by_field_name('value')
            if receiver is None or receiver.type != 'identifier':
                continue
            if self.node_text(receiver, code) == var_name:
                fn_args.append(mcall)

        # Collect reassignments: var = ...
        for assign in self.find_nodes(block, 'assignment_expression'):
            left = assign.child_by_field_name('left')
            if left is not None and left.type == 'identifier':
                if self.node_text(left, code).strip() == var_name:
                    reassignments.append(assign)

        return BindingUsages(
            field_derefs=field_derefs,
            raw_derefs=raw_derefs,
            fn_args=fn_args,
            reassignments=reassignments,
            compound_assigns=compound_assigns,
            method_calls=method_calls,
        )

    def _is_mutably_used_field_expr(self, fe: Any, code: str) -> bool:
        """True if the field expression is on the LHS of an assignment or compound-assign."""
        cur = fe
        while cur is not None:
            parent = cur.parent
            if parent is None:
                break
            if parent.type in ('assignment_expression', 'compound_assignment_expr'):
                left = parent.child_by_field_name('left')
                if left is not None:
                    if left.start_byte <= fe.start_byte and fe.end_byte <= left.end_byte:
                        return True
            elif parent.type == 'reference_expression':
                for ch in parent.children:
                    if ch.type == 'mutable_specifier':
                        return True
            elif parent.type == 'block':
                break
            cur = parent
        return False

    # ── Pattern A: cast_binding_to_safe_ref ─────────────────────────────────

    def _collect_cast_binding_matches(self, code: str) -> List[Dict[str, Any]]:
        """
        Find let bindings of the form:
          let [mut] p: *mut T = EXPR as *mut T;
          ... (*p).field ...   (all uses are field derefs)

        Returns list of transform descriptors.
        """
        root = self.parse(code)
        all_field_exprs = self.find_nodes(root, 'field_expression')

        # Group field derefs by (block_start, ptr_var)
        groups: Dict[Tuple[int, str], Dict[str, Any]] = {}
        for fe in all_field_exprs:
            ptr_var = self._is_direct_deref_field(fe, code)
            if ptr_var is None:
                continue
            if not self._is_extern_c_unsafe_function(fe, code):
                continue
            block = self.get_parent_of_type(fe, 'block')
            if block is None:
                continue
            key = (block.start_byte, ptr_var)
            if key not in groups:
                groups[key] = {'block': block, 'ptr_var': ptr_var, 'field_exprs': []}
            groups[key]['field_exprs'].append(fe)

        results = []
        for (block_start, ptr_var), g in groups.items():
            block = g['block']

            # Must have a cast binding in this exact block
            binding = self._find_cast_binding(block, ptr_var, code)
            if binding is None:
                continue

            # Skip cast bindings whose RHS contains (*ptr).field patterns.
            # Those will be handled by raw_ptr_deref_field_chain first, and
            # transforming them here would create new (*ptr).field occurrences
            # that break idempotency.
            if DEREF_FIELD_RE.search(binding.rhs_expr):
                continue

            # Full data-flow analysis on all usages
            usages = self._collect_usages(block, ptr_var, code)

            # Only transform if the variable is ONLY used via (*var).field
            if not usages.only_field_derefs:
                continue

            # At least 1 use (field deref or method call) needed
            if not usages.field_derefs and not usages.method_calls:
                continue

            # Determine mutability requirement.
            # Method calls on a *mut pointer conservatively require &mut.
            needs_mut = (
                binding.is_mut and (
                    any(self._is_mutably_used_field_expr(fe, code)
                        for fe in usages.field_derefs)
                    or bool(usages.compound_assigns)
                    or bool(usages.method_calls)
                )
            )

            results.append({
                'pattern': 'cast_binding_to_safe_ref',
                'block': block,
                'ptr_var': ptr_var,
                'binding': binding,
                'field_exprs': usages.field_derefs + usages.compound_assigns + usages.method_calls,
                'needs_mut': needs_mut,
            })

        return results

    # ── Pattern B: nested_deref_hoist ───────────────────────────────────────

    def _collect_nested_deref_matches(self, code: str) -> List[Dict[str, Any]]:
        """
        Find repeated (*(*ptr).inner_ptr).subfield patterns.
        Groups by (block, outer_var, inner_field) and requires ≥ 2 uses.

        After collecting per-block matches, deduplicates: for any two matches
        with the same (outer_var, inner_field) where block A is contained in
        block B, keep only block B (outermost).  This prevents nested blocks
        from producing overlapping replacements that corrupt the code.
        """
        root = self.parse(code)
        all_field_exprs = self.find_nodes(root, 'field_expression')

        # Original per-block grouping
        groups: Dict[Tuple[int, str, str], Dict[str, Any]] = {}

        for fe in all_field_exprs:
            nested = self._is_nested_deref_field(fe, code)
            if nested is None:
                continue
            outer_var, inner_field = nested

            if not self._is_extern_c_unsafe_function(fe, code):
                continue

            block = self.get_parent_of_type(fe, 'block')
            if block is None:
                continue

            key = (block.start_byte, outer_var, inner_field)
            if key not in groups:
                groups[key] = {
                    'block': block,
                    'outer_var': outer_var,
                    'inner_field': inner_field,
                    'uses': [],
                    'needs_mut': False,
                }
            groups[key]['uses'].append(fe)
            if self._is_mutably_used_field_expr(fe, code):
                groups[key]['needs_mut'] = True

        MIN_USES = 2
        candidates = []
        for (block_start, outer_var, inner_field), g in groups.items():
            if len(g['uses']) < MIN_USES:
                continue

            # Find byte of first nested deref use
            first_use_byte = min(fe.start_byte for fe in g['uses'])

            if self._has_ptr_reassignment(g['block'], outer_var, code,
                                          after_byte=first_use_byte):
                continue

            # Skip if the outer_var is null-checked anywhere in the block.
            # Inserting a dereference before a null-check causes a segfault
            # when the pointer is actually null.
            block_text = self.node_text(g['block'], code)
            if re.search(rf'\b{re.escape(outer_var)}\s*\.\s*is_null\s*\(', block_text):
                continue

            bind_name = f'__{inner_field}_ref'
            if f'let {bind_name}' in block_text or f'let mut {bind_name}' in block_text:
                continue

            candidates.append({
                'pattern': 'nested_deref_hoist',
                'block': g['block'],
                'outer_var': outer_var,
                'inner_field': inner_field,
                'bind_name': bind_name,
                'uses': g['uses'],
                'needs_mut': g['needs_mut'],
            })

        # Deduplicate: for the same (outer_var, inner_field), if block A is
        # strictly contained in block B, keep only block B (outermost).  This
        # prevents overlapping replacements when nested blocks both qualify.
        results = []
        for i, cand in enumerate(candidates):
            dominated = False
            for j, other in enumerate(candidates):
                if i == j:
                    continue
                if (cand['outer_var'] == other['outer_var'] and
                        cand['inner_field'] == other['inner_field']):
                    # Is cand's block strictly inside other's block?
                    if (other['block'].start_byte <= cand['block'].start_byte and
                            cand['block'].end_byte <= other['block'].end_byte and
                            other['block'].start_byte != cand['block'].start_byte):
                        dominated = True
                        break
            if not dominated:
                results.append(cand)

        return results

    def _has_ptr_reassignment(self, block: Any, ptr_var: str, code: str,
                              after_byte: int = 0) -> bool:
        """
        Check if ptr_var is reassigned in block at or after after_byte.
        If after_byte=0, checks the entire block.
        This prevents false positives when ptr is assigned BEFORE its first use
        (common C pattern: declare null, then malloc, then use).
        """
        for assign in self.find_nodes(block, 'assignment_expression'):
            if assign.start_byte < after_byte:
                continue
            left = assign.child_by_field_name('left')
            if left is not None and left.type == 'identifier':
                if self.node_text(left, code).strip() == ptr_var:
                    return True
        return False

    def _collect_all_matches(self, code: str) -> List[Dict[str, Any]]:
        """Collect all pattern matches across both sub-patterns."""
        matches = []
        matches.extend(self._collect_cast_binding_matches(code))
        matches.extend(self._collect_nested_deref_matches(code))
        return matches

    # ── Quick pre-filter ─────────────────────────────────────────────────────

    @staticmethod
    def _quick_check(rust_code: str) -> bool:
        """Fast pre-filter to avoid expensive AST analysis on non-matching files."""
        # Any (*VAR).field pattern in file with extern
        return (bool(DEREF_FIELD_RE.search(rust_code)) and
                ('unsafe' in rust_code or 'extern' in rust_code))

    # ── find() ───────────────────────────────────────────────────────────────

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        if not self._quick_check(rust_code):
            return []

        code_hash = hashlib.sha256(rust_code.encode()).hexdigest()
        find_cache = os.path.join(_CACHE_DIR, f"find_{code_hash}.json")
        if os.path.exists(find_cache):
            try:
                with open(find_cache, 'r', encoding='utf-8') as f:
                    return [tuple(r) for r in json.load(f)]
            except Exception:
                pass

        results = []
        for m in self._collect_all_matches(rust_code):
            block = m['block']
            first_byte = block.start_byte
            line = rust_code[:first_byte].count('\n') + 1

            if m['pattern'] == 'cast_binding_to_safe_ref':
                n = len(m['field_exprs'])
                mut_str = "mut " if m['needs_mut'] else ""
                desc = (
                    f"Line {line}: cast_binding_to_safe_ref — "
                    f"`let {mut_str}{m['ptr_var']}` cast binding with {n} field "
                    f"deref(s) → hoist to `&{mut_str}*({m['binding'].rhs_expr})`"
                )
            else:
                n = len(m['uses'])
                mut_str = "mut " if m['needs_mut'] else ""
                desc = (
                    f"Line {line}: nested_deref_hoist — "
                    f"`(*(*{m['outer_var']}).{m['inner_field']}).sub` used {n}× "
                    f"→ hoist `let {m['bind_name']} = &{mut_str}*(*{m['outer_var']}).{m['inner_field']}`"
                )
            results.append((block.start_byte, block.end_byte, desc))

        try:
            os.makedirs(_CACHE_DIR, exist_ok=True)
            with open(find_cache, 'w', encoding='utf-8') as f:
                json.dump(results, f)
        except Exception:
            pass

        return results

    # ── fix() ────────────────────────────────────────────────────────────────

    def fix(self, rust_code: str) -> str:
        if not self._quick_check(rust_code):
            return rust_code

        code_hash = hashlib.sha256(rust_code.encode()).hexdigest()
        fix_cache = os.path.join(_CACHE_DIR, f"fix_{code_hash}.txt")
        if os.path.exists(fix_cache):
            try:
                with open(fix_cache, 'r', encoding='utf-8') as f:
                    return f.read()
            except Exception:
                pass

        MAX_ROUNDS = 15
        for _round in range(MAX_ROUNDS):
            matches = self._collect_all_matches(rust_code)
            if not matches:
                break
            rust_code = self._apply_all_matches(rust_code, matches)

        try:
            os.makedirs(_CACHE_DIR, exist_ok=True)
            with open(fix_cache, 'w', encoding='utf-8') as f:
                f.write(rust_code)
        except Exception:
            pass

        return rust_code

    def _apply_all_matches(self, rust_code: str, matches: List[Dict[str, Any]]) -> str:
        """Apply one round of fixes, grouped by block to avoid conflicts."""
        # Sort by block start byte descending so that replacements don't shift offsets
        by_block: Dict[int, List[Dict[str, Any]]] = defaultdict(list)
        for m in matches:
            by_block[m['block'].start_byte].append(m)

        # Filter out matches whose blocks are strictly nested inside another
        # match's block. Nested matches would produce overlapping byte-range
        # replacements that corrupt the code. The fix() loop re-runs this
        # method on subsequent rounds, so nested patterns are handled then.
        all_ranges = [
            (by_block[s][0]['block'].start_byte, by_block[s][0]['block'].end_byte)
            for s in by_block
        ]
        def is_nested_in_other(start: int, end: int) -> bool:
            return any(
                outer_s < start and end <= outer_e
                for outer_s, outer_e in all_ranges
                if outer_s != start
            )
        by_block = {
            s: ms for s, ms in by_block.items()
            if not is_nested_in_other(
                ms[0]['block'].start_byte, ms[0]['block'].end_byte
            )
        }

        replacements: List[Tuple[int, int, str]] = []

        for block_start, block_matches in by_block.items():
            block = block_matches[0]['block']
            block_text = self.node_text(block, rust_code)
            block_lines = block_text.split('\n')

            # Detect indentation from first statement
            indent = "    "
            for ln in block_lines[1:]:
                stripped = ln.lstrip()
                if stripped and stripped not in ('{', '}'):
                    indent = ln[:len(ln) - len(stripped)]
                    break

            # Process each match in this block
            # Snapshot original lines for offset arithmetic
            orig_lines = block_lines[:]
            # All bindings to insert: (after_line, binding_text)
            insertions: List[Tuple[int, str]] = []

            for m in block_matches:
                if m['pattern'] == 'cast_binding_to_safe_ref':
                    block_lines, new_insertions = self._apply_cast_binding(
                        block_lines, orig_lines, block, m, rust_code, indent
                    )
                    insertions.extend(new_insertions)
                elif m['pattern'] == 'nested_deref_hoist':
                    block_lines, new_insertions = self._apply_nested_hoist(
                        block_lines, orig_lines, block, m, rust_code, indent
                    )
                    insertions.extend(new_insertions)

            # Insert bindings bottom-to-top to preserve line numbers
            for after_line, binding_text in sorted(insertions, key=lambda x: x[0], reverse=True):
                block_lines.insert(after_line + 1, binding_text)

            new_block = '\n'.join(block_lines)
            replacements.append((block.start_byte, block.end_byte, new_block))

        return self.apply_replacements(rust_code, replacements)

    def _apply_cast_binding(
        self,
        block_lines: List[str],
        orig_lines: List[str],
        block: Any,
        match: Dict[str, Any],
        code: str,
        indent: str,
    ) -> Tuple[List[str], List[Tuple[int, str]]]:
        """
        Apply Pattern A transform: replace cast binding declaration + all (*var).field
        with a safe reference binding + var.field.

        Returns updated block_lines and list of (after_line, binding_text) insertions.
        """
        binding: CastBinding = match['binding']
        ptr_var = match['ptr_var']
        needs_mut = match['needs_mut']
        mut_str = "mut " if needs_mut else ""

        # Step 1: Remove the original let declaration
        decl_text = self.node_text(binding.decl_node, code)
        # Find the line containing the declaration in block_lines
        # Use the node's offset relative to block start
        decl_offset = binding.decl_node.start_byte - block.start_byte
        decl_line_idx = self._byte_offset_to_line_index(orig_lines, decl_offset)

        # Build the new safe reference binding
        # Format: let [mut] p = &[mut] *(ORIGINAL_RHS);
        new_binding = f"{indent}let {mut_str}{ptr_var} = &{mut_str}*({binding.rhs_expr});"

        # Replace the declaration — which may span multiple lines.
        # Compute the end-line index too, then blank continuation lines.
        decl_end_offset = binding.decl_node.end_byte - block.start_byte
        decl_end_line_idx = self._byte_offset_to_line_index(orig_lines, decl_end_offset - 1)
        if decl_line_idx is not None:
            block_lines[decl_line_idx] = new_binding
            # Blank out any continuation lines of the original multi-line decl
            end_idx = decl_end_line_idx if decl_end_line_idx is not None else decl_line_idx
            for i in range(decl_line_idx + 1, end_idx + 1):
                if i < len(block_lines):
                    block_lines[i] = ""
        else:
            # Fallback: text replacement
            block_lines_text = '\n'.join(block_lines)
            block_lines_text = block_lines_text.replace(decl_text, new_binding.lstrip(), 1)
            block_lines = block_lines_text.split('\n')

        # Step 2: Replace all (*var).field with var.field in block_lines
        deref_re = re.compile(rf'\(\*{re.escape(ptr_var)}\)\s*\.')
        block_lines = [deref_re.sub(f'{ptr_var}.', ln) for ln in block_lines]

        return block_lines, []

    def _apply_nested_hoist(
        self,
        block_lines: List[str],
        orig_lines: List[str],
        block: Any,
        match: Dict[str, Any],
        code: str,
        indent: str,
    ) -> Tuple[List[str], List[Tuple[int, str]]]:
        """
        Apply Pattern B transform: hoist inner pointer from (*(*p).inner).sub patterns.

        Returns updated block_lines and list of (after_line, binding_text) insertions.
        """
        outer_var = match['outer_var']
        inner_field = match['inner_field']
        bind_name = match['bind_name']
        needs_mut = match['needs_mut']
        mut_str = "mut " if needs_mut else ""

        # Build the binding: let [mut] __inner_ref = &[mut] *(*outer_var).inner_field;
        binding_text = f"{indent}let {bind_name} = &{mut_str}*(*{outer_var}).{inner_field};"

        # Replace (*(*outer_var).inner_field).sub with bind_name.sub
        # Use DOTALL so \s* matches newlines — handles multi-line expressions like:
        #   (*(*p).pFts3Tab)\n    .zDb  →  __pFts3Tab_ref.zDb
        nested_re = re.compile(
            rf'\(\*\(\*{re.escape(outer_var)}\)\s*\.\s*{re.escape(inner_field)}\)\s*\.',
            re.DOTALL,
        )
        joined = '\n'.join(block_lines)
        joined = nested_re.sub(f'{bind_name}.', joined)
        block_lines = joined.split('\n')

        # Find insertion point: insert BEFORE the direct-child statement of the
        # block that contains the first use.  Using "line before first match"
        # can land inside a multi-line expression (e.g. multi-line if condition)
        # which is invalid Rust.  Instead walk the AST.
        uses = match.get('uses', [])
        block = match['block']
        insert_after_line = 0  # fallback: right after opening '{'

        if uses:
            first_use = min(uses, key=lambda fe: fe.start_byte)
            # Walk up to the direct child of block
            stmt_node = first_use
            while (stmt_node.parent is not None and
                   stmt_node.parent.start_byte != block.start_byte):
                stmt_node = stmt_node.parent
            stmt_offset = stmt_node.start_byte - block.start_byte
            stmt_line = self._byte_offset_to_line_index(orig_lines, stmt_offset)
            if stmt_line is not None:
                # Insert after the line BEFORE the statement (i.e., before it)
                insert_after_line = max(0, stmt_line - 1)

        return block_lines, [(insert_after_line, binding_text)]

    @staticmethod
    def _byte_offset_to_line_index(lines: List[str], byte_offset: int) -> Optional[int]:
        """Convert a byte offset within a block to a 0-based line index."""
        char_count = 0
        for i, ln in enumerate(lines):
            char_count += len(ln) + 1  # +1 for the newline
            if char_count > byte_offset:
                return i
        return None

    # ── Tests ─────────────────────────────────────────────────────────────────

    def test(self) -> bool:
        all_passed = True

        def check(name: str, condition: bool, msg: str = "") -> None:
            nonlocal all_passed
            if condition:
                print(f"  ✓ {name}")
            else:
                print(f"  ✗ {name}: {msg}")
                all_passed = False

        # ── Pattern A tests ──

        # Test A1: Basic cast binding conversion
        code_a1 = '\n'.join([
            "unsafe extern \"C\" fn fts3auxRowidMethod(",
            "    mut pCursor: *mut sqlite3_vtab_cursor,",
            "    mut pRowid: *mut sqlite_int64,",
            ") -> c_int {",
            "    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;",
            "    *pRowid = (*pCsr).iRowid as sqlite_int64;",
            "    return SQLITE_OK;",
            "}",
        ])
        finds_a1 = self.find(code_a1)
        check("A1 cast_binding detection", len(finds_a1) >= 1,
              f"expected >= 1, got {len(finds_a1)}")

        fixed_a1 = self.fix(code_a1)
        check("A1 safe ref created", "&*(" in fixed_a1, f"no &*( found:\n{fixed_a1}")
        check("A1 raw deref removed", "(*pCsr)." not in fixed_a1,
              f"(*pCsr). still present:\n{fixed_a1}")
        check("A1 pCsr.iRowid present", "pCsr.iRowid" in fixed_a1,
              f"pCsr.iRowid missing:\n{fixed_a1}")
        check("A1 idempotent", self.fix(fixed_a1) == fixed_a1,
              f"second pass changed code:\n{self.fix(fixed_a1)}")

        # Test A2: Another single deref (fts3auxEofMethod-like)
        code_a2 = '\n'.join([
            "unsafe extern \"C\" fn fts3auxEofMethod(",
            "    mut pCursor: *mut sqlite3_vtab_cursor,",
            ") -> c_int {",
            "    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;",
            "    return (*pCsr).isEof;",
            "}",
        ])
        finds_a2 = self.find(code_a2)
        check("A2 single-use detection", len(finds_a2) >= 1,
              f"expected >= 1, got {len(finds_a2)}")
        fixed_a2 = self.fix(code_a2)
        check("A2 safe ref", "&*(" in fixed_a2, f"\n{fixed_a2}")
        check("A2 pCsr.isEof", "pCsr.isEof" in fixed_a2, f"\n{fixed_a2}")

        # Test A3: Variable passed to function — MUST NOT transform
        code_a3 = '\n'.join([
            "unsafe extern \"C\" fn example(",
            "    pCursor: *mut sqlite3_vtab_cursor,",
            ") -> c_int {",
            "    let pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;",
            "    let x = (*pCsr).field;",
            "    other_function(pCsr);",  # <-- escapes to fn
            "    return 0;",
            "}",
        ])
        fixed_a3 = self.fix(code_a3)
        check("A3 no transform when escaping to fn",
              "(*pCsr).field" in fixed_a3,
              f"was transformed but shouldn't have been:\n{fixed_a3}")

        # Test A4: Mutable binding — needs &mut *
        code_a4 = '\n'.join([
            "unsafe extern \"C\" fn example(p: *mut sqlite3_vtab) -> c_int {",
            "    let mut pAux: *mut AuxTable = p as *mut AuxTable;",
            "    (*pAux).nRef += 1;",
            "    return (*pAux).nRef;",
            "}",
        ])
        finds_a4 = self.find(code_a4)
        check("A4 mutable detection", len(finds_a4) >= 1,
              f"expected >= 1, got {len(finds_a4)}")
        fixed_a4 = self.fix(code_a4)
        check("A4 mut ref created", "&mut *(" in fixed_a4,
              f"no &mut *( found:\n{fixed_a4}")
        check("A4 field access safe", "pAux.nRef" in fixed_a4,
              f"pAux.nRef missing:\n{fixed_a4}")

        # Test A5: Not extern C — should not transform
        code_a5 = '\n'.join([
            "unsafe fn internal_only(p: *mut sqlite3_vtab) -> c_int {",
            "    let pCsr: *mut Fts3auxCursor = p as *mut Fts3auxCursor;",
            "    return (*pCsr).isEof;",
            "}",
        ])
        fixed_a5 = self.fix(code_a5)
        check("A5 no transform for non-extern",
              "(*pCsr).isEof" in fixed_a5,
              f"was transformed but shouldn't:\n{fixed_a5}")

        # ── Pattern B tests ──

        # Test B1: Basic nested deref hoist (fts3auxConnectMethod pattern)
        code_b1 = '\n'.join([
            "unsafe extern \"C\" fn connect(p: *mut AuxTable) -> c_int {",
            "    (*(*p).pFts3Tab).zDb = ptr1;",
            "    (*(*p).pFts3Tab).zName = ptr2;",
            "    (*(*p).pFts3Tab).nIndex = 1;",
            "    return SQLITE_OK;",
            "}",
        ])
        finds_b1 = self.find(code_b1)
        check("B1 nested_deref detection", len(finds_b1) >= 1,
              f"expected >= 1, got {len(finds_b1)}")
        fixed_b1 = self.fix(code_b1)
        check("B1 binding created",
              "__pFts3Tab_ref" in fixed_b1,
              f"binding missing:\n{fixed_b1}")
        check("B1 raw deref removed",
              "(*(*p).pFts3Tab)." not in fixed_b1,
              f"raw deref still present:\n{fixed_b1}")
        check("B1 field access preserved",
              "__pFts3Tab_ref.zDb" in fixed_b1 and "__pFts3Tab_ref.nIndex" in fixed_b1,
              f"field access missing:\n{fixed_b1}")

        # Test B2: Only 1 use — should not hoist (below threshold)
        code_b2 = '\n'.join([
            "unsafe extern \"C\" fn connect(p: *mut AuxTable) -> c_int {",
            "    (*(*p).pFts3Tab).zDb = ptr1;",
            "    return SQLITE_OK;",
            "}",
        ])
        fixed_b2 = self.fix(code_b2)
        check("B2 no hoist below threshold",
              "(*(*p).pFts3Tab).zDb" in fixed_b2,
              f"was hoisted below threshold:\n{fixed_b2}")

        # Test B3: Idempotency for nested hoist
        check("B3 nested hoist idempotent",
              self.fix(fixed_b1) == fixed_b1,
              f"second pass changed:\n{self.fix(fixed_b1)}")

        return all_passed
