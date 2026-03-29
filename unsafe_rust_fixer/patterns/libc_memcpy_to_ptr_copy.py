"""Plugin: Replace libc::memcpy/memmove with core::ptr::copy_nonoverlapping/copy.

C-to-Rust transpilers emit C FFI calls to memory functions that should be
replaced with idiomatic Rust core::ptr operations.

-- memcpy with typed inner cast (type-preserving mode) --

Before:
    ::libc::memcpy(
        buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        n as crate::__stddef_size_t_h::size_t,
    );

After (type preserved — both sides share the same 1-byte type):
    ::core::ptr::copy_nonoverlapping(
        src as *const ::core::ffi::c_char,
        buf as *mut ::core::ffi::c_char,
        n as usize,
    );

-- memcpy with bare inner expression (u8 fallback mode) --

Before:
    ::libc::memcpy(
        buf as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        n as crate::__stddef_size_t_h::size_t,
    );

After (u8 fallback — no shared inner type):
    ::core::ptr::copy_nonoverlapping(
        src as *const u8,
        buf as *mut u8,
        n as usize,
    );

Key transformations:
  1. Argument order reversal: C (dst, src, n) → Rust (src, dst, n)
  2. Void pointer casts stripped; element type chosen by type-inference:
       a. If both dst and src inner expressions end with the SAME 1-byte type alias,
          that type is used for the copy element type — no extra cast appended.
       b. Otherwise fall back to u8 (safe universal byte type).
  3. Size type changed: any size_t / wrapping_mul / bare expr → usize
  4. memcpy maps to copy_nonoverlapping (no overlap); memmove maps to copy (overlap ok)

Known 1-byte type aliases handled:
  - ::core::ffi::c_char         (= i8)
  - ::core::ffi::c_uchar        (= u8) → canonical u8
  - u8_0 (short)                (= u8) → canonical u8
  - crate::src::ext::rtree::rtree::u8_0  → canonical u8
  - crate::src::fts5::u8_0      → canonical u8
  - u8

Both dst and src must have the same CANONICAL type for type-preserving mode.
c_char vs c_uchar differ canonically (i8 vs u8), so mixing them falls back to u8.

Both replacement functions still require an enclosing `unsafe` block, but they:
  - Eliminate the libc FFI dependency for basic memory operations
  - Preserve semantic type information when available (c_char stays c_char)
  - Express non-overlap / overlap semantics in the function name

Data-flow analysis:
  - Tree-sitter locates call_expression nodes and inspects argument AST types
  - The void-pointer cast structure is verified at the AST level
  - Byte-sized inner type is inferred from trailing cast in inner expression
  - Replacements are applied in reverse byte order (no offset corruption)
"""

import re
from typing import List, Tuple, Optional, Any

from .base import UnsafePatternPlugin



class LibcMemcpyToPtrCopyPlugin(UnsafePatternPlugin):
    """Replace ::libc::memcpy / ::libc::memmove with core::ptr analogues.

    Detects the canonical C-to-Rust transpiler pattern:
        ::libc::memcpy(dst as *mut c_void, src as *const c_void, n as size_t)
    and rewrites it to:
        ::core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, n as usize)

    Handles:
      - Simple pointer-to-void casts: `ptr as *mut ::core::ffi::c_void`
      - Multi-level casts:           `ptr as *mut c_uchar as *mut ::core::ffi::c_void`
      - size_of expressions:         `::core::mem::size_of::<T>() as size_t`
      - Both memcpy and memmove variants
    """

    @property
    def name(self) -> str:
        return "libc_memcpy_to_ptr_copy"

    @property
    def description(self) -> str:
        return (
            "Replace ::libc::memcpy/memmove with ::core::ptr::copy_nonoverlapping/copy; "
            "reverses arg order (dst,src,n → src,dst,n), strips void casts, "
            "preserves inner byte-type aliases (c_char, u8_0, c_uchar → typed copy), "
            "converts size_t → usize"
        )

    @property
    def priority(self) -> int:
        return 11  # High — 400+ instances; structural arg-order + type transformation

    # ── private helpers ───────────────────────────────────────────────────────

    _MEMCPY_FN = "::libc::memcpy"
    _MEMMOVE_FN = "::libc::memmove"

    # Void pointer type suffixes we accept on the first two arguments.
    _VOID_MUT_MARKERS = (
        "*mut ::core::ffi::c_void",
        "*mut c_void",
    )
    _VOID_CONST_MARKERS = (
        "*const ::core::ffi::c_void",
        "*const c_void",
    )

    # Mapping from known 1-byte type strings → canonical form.
    # Used by type-inference: two inner expressions sharing the same canonical type
    # can use that type directly as the copy element type (no extra u8 cast needed).
    # Only 1-byte types are safe: copy_nonoverlapping count is in elements, not bytes.
    # Multi-byte types (u32_0, c_int, WalIndexHdr, …) are intentionally absent —
    # they fall back to u8 so the byte-count semantics are preserved.
    _BYTE_TYPE_CANONICAL: dict = {
        # Unsigned byte
        "u8":                                          "u8",
        "u8_0":                                        "u8",   # short alias
        "::core::ffi::c_uchar":                        "u8",
        "crate::src::ext::rtree::rtree::u8_0":         "u8",
        "crate::src::fts5::u8_0":                      "u8",
        # Signed byte
        "i8":                                          "i8",
        "::core::ffi::c_char":                         "::core::ffi::c_char",
        "::core::ffi::c_schar":                        "i8",
    }

    # Regex to extract the trailing `as *{mut|const} TYPE` from an inner expression.
    _TRAILING_PTR_CAST_RE = re.compile(
        r'\bas\s+\*(?:mut|const)\s+(\S+)\s*$'
    )

    def _infer_element_type(self, dst_inner: str, src_inner: str) -> str:
        """Return the best element type for copy_nonoverlapping given the two inner expressions.

        Rules:
          1. Extract the trailing `as *{mut|const} T` from each inner expression.
          2. Look up T in _BYTE_TYPE_CANONICAL (only 1-byte types are in this map).
          3. If BOTH map to the same canonical type: return that canonical type string.
          4. Otherwise return "u8" (safe universal fallback).

        Using a matched canonical type allows the replacement to drop the extra
        `as *mut u8` / `as *const u8` suffix and keep the more specific type,
        producing cleaner and more semantically accurate output.
        """
        m_dst = self._TRAILING_PTR_CAST_RE.search(dst_inner.strip())
        m_src = self._TRAILING_PTR_CAST_RE.search(src_inner.strip())

        if m_dst is None or m_src is None:
            return "u8"

        canon_dst = self._BYTE_TYPE_CANONICAL.get(m_dst.group(1))
        canon_src = self._BYTE_TYPE_CANONICAL.get(m_src.group(1))

        if canon_dst is None or canon_src is None:
            return "u8"

        if canon_dst != canon_src:
            return "u8"

        return canon_dst

    def _format_src_arg(self, src_inner: str, elem_type: str) -> str:
        """Format the src argument for copy_nonoverlapping.

        If elem_type was inferred from the inner cast:
          - src_inner already ends with `as *{mut|const} T`; replace with `*const T`
            so Rust sees the correct const-ness without an extra cast.
        Otherwise (u8 fallback):
          - Append `as *const u8`.
        """
        if elem_type == "u8":
            return f"{src_inner} as *const u8"
        # Replace the trailing mut/const pointer cast with `*const elem_type`
        replaced = self._TRAILING_PTR_CAST_RE.sub(
            f"as *const {elem_type}", src_inner.strip()
        )
        return replaced

    def _format_dst_arg(self, dst_inner: str, elem_type: str) -> str:
        """Format the dst argument for copy_nonoverlapping.

        If elem_type was inferred: the trailing `as *mut T` is already correct;
        just return dst_inner as-is (it ends with the right pointer type).
        Otherwise: append `as *mut u8`.
        """
        if elem_type == "u8":
            return f"{dst_inner} as *mut u8"
        # The trailing cast in dst_inner is already `as *mut T`; keep it.
        return dst_inner.strip()

    def _get_call_args(self, call_node: Any) -> List[Any]:
        """Return the non-punctuation children of the arguments node."""
        args_node = call_node.child_by_field_name("arguments")
        if args_node is None:
            return []
        return [c for c in args_node.children if c.type not in ("(", ")", ",")]

    def _is_fn_match(self, call_node: Any, code: str) -> Optional[str]:
        """Return the matched function name, or None if not a match."""
        fn_node = call_node.child_by_field_name("function")
        if fn_node is None:
            return None
        fn_text = self.node_text(fn_node, code).strip()
        if fn_text in (self._MEMCPY_FN, self._MEMMOVE_FN):
            return fn_text
        return None

    def _extract_void_cast_inner(
        self, node: Any, code: str, markers: Tuple[str, ...]
    ) -> Optional[str]:
        """If *node* is `INNER as *{mut|const} c_void`, return the text of INNER.

        Only succeeds when the outermost cast targets one of the given void-pointer
        type markers.  Multi-level casts (inner expressions that are themselves
        casts) are preserved verbatim — only the outermost void cast is stripped.

        Returns None when the node is not a matching void-pointer cast.
        """
        if node.type not in ("type_cast_expression", "cast_expression"):
            return None

        type_node = node.child_by_field_name("type")
        if type_node is None:
            return None

        type_text = self.node_text(type_node, code).strip()
        if not any(type_text == m or type_text.endswith(m) for m in markers):
            return None

        value_node = node.child_by_field_name("value")
        if value_node is None:
            return None

        return self.node_text(value_node, code)

    def _extract_size_expr(self, node: Any, code: str) -> str:
        """Return a usize-typed size expression from the third argument.

        Strategy:
          - If the outermost AST node is a cast to a size_t or usize type,
            strip that cast and add `as usize` — avoids double-casting.
          - For any other expression shape (e.g. call_expression via .wrapping_mul,
            parenthesized_expression, etc.) wrap the entire text in parentheses
            and append `as usize`.  This is always safe: the original value is
            already a byte count, so casting to usize is a no-op on 64-bit and
            a value-preserving truncation on 32-bit — identical semantics to the
            original `as size_t` implicit in libc::memcpy.

        The REGEX fallback previously used here was removed because it matched
        `size_t` tokens INSIDE nested calls (e.g. `.wrapping_mul(n as size_t)`)
        and incorrectly stripped characters from the middle of the expression,
        producing broken Rust code.
        """
        node_text = self.node_text(node, code).strip()

        # AST-only: only act on the OUTERMOST cast node
        if node.type in ("type_cast_expression", "cast_expression"):
            type_node = node.child_by_field_name("type")
            if type_node is not None:
                type_text = self.node_text(type_node, code).strip()
                if "size_t" in type_text or "usize" in type_text:
                    value_node = node.child_by_field_name("value")
                    if value_node is not None:
                        inner = self.node_text(value_node, code).strip()
                        return f"{inner} as usize"

        # For all other expression shapes (call_expression, parenthesized_expression,
        # binary_expression, etc.), wrap the entire expression.  The extra parens
        # are syntactically harmless and ensure the `as usize` binds to the whole
        # expression, not just the last token.
        return f"({node_text}) as usize"

    def _build_replacement(
        self, fn_name: str, dst_inner: str, src_inner: str, size_expr: str
    ) -> str:
        """Build the replacement Rust expression string.

        The element type is inferred from the inner expressions (type-preserving mode)
        or falls back to u8 when types differ or are multi-byte.

        memcpy  → ::core::ptr::copy_nonoverlapping(src_arg, dst_arg, n)
        memmove → ::core::ptr::copy(src_arg, dst_arg, n)
        """
        rust_fn = (
            "::core::ptr::copy_nonoverlapping"
            if fn_name == self._MEMCPY_FN
            else "::core::ptr::copy"
        )
        elem_type = self._infer_element_type(dst_inner, src_inner)
        src_arg = self._format_src_arg(src_inner, elem_type)
        dst_arg = self._format_dst_arg(dst_inner, elem_type)

        return (
            f"{rust_fn}(\n"
            f"                    {src_arg},\n"
            f"                    {dst_arg},\n"
            f"                    {size_expr},\n"
            f"                )"
        )

    # ── public interface ──────────────────────────────────────────────────────

    def find(self, rust_code: str) -> List[Tuple[int, int, str]]:
        """Find all transformable ::libc::memcpy / ::libc::memmove call sites.

        Returns (start_byte, end_byte, description) for each match.
        Only reports calls where all three argument shapes are verified:
          - arg0: cast to *mut c_void  (destination)
          - arg1: cast to *const c_void (source)
          - arg2: any expression (size, may or may not have size_t cast)
        """
        root = self.parse(rust_code)
        call_nodes = self.find_nodes(root, "call_expression")
        results: List[Tuple[int, int, str]] = []

        for call in call_nodes:
            fn_name = self._is_fn_match(call, rust_code)
            if fn_name is None:
                continue

            args = self._get_call_args(call)
            if len(args) != 3:
                continue

            dst_inner = self._extract_void_cast_inner(
                args[0], rust_code, self._VOID_MUT_MARKERS
            )
            src_inner = self._extract_void_cast_inner(
                args[1], rust_code, self._VOID_CONST_MARKERS
            )

            if dst_inner is None or src_inner is None:
                # Not the expected pattern (void-pointer cast form); skip
                continue

            rust_fn = (
                "copy_nonoverlapping" if fn_name == self._MEMCPY_FN else "copy"
            )
            line = self.node_line(call, rust_code)
            results.append((
                call.start_byte,
                call.end_byte,
                (
                    f"Line {line}: {fn_name}(...) — replace with "
                    f"::core::ptr::{rust_fn}(src, dst, n as usize) "
                    f"[arg order reversed, void casts stripped]"
                ),
            ))

        return results

    def fix(self, rust_code: str) -> str:
        """Apply all memcpy/memmove → ptr::copy_* transformations.

        Processes in reverse byte order so earlier replacements do not
        corrupt the byte offsets of later ones.
        """
        root = self.parse(rust_code)
        call_nodes = self.find_nodes(root, "call_expression")
        replacements: List[Tuple[int, int, str]] = []

        for call in call_nodes:
            fn_name = self._is_fn_match(call, rust_code)
            if fn_name is None:
                continue

            args = self._get_call_args(call)
            if len(args) != 3:
                continue

            dst_inner = self._extract_void_cast_inner(
                args[0], rust_code, self._VOID_MUT_MARKERS
            )
            src_inner = self._extract_void_cast_inner(
                args[1], rust_code, self._VOID_CONST_MARKERS
            )

            if dst_inner is None or src_inner is None:
                continue

            size_expr = self._extract_size_expr(args[2], rust_code)

            replacement = self._build_replacement(
                fn_name, dst_inner, src_inner, size_expr
            )
            replacements.append((call.start_byte, call.end_byte, replacement))

        return self.apply_replacements(rust_code, replacements)

    def test(self) -> bool:
        """Self-contained regression tests for this plugin."""

        # ── Test 1: basic memcpy with simple pointer casts ──────────────────
        code1 = r"""
unsafe fn copy_page(zOut: *mut u8, zIn: *const u8, nCopy: i32) {
    ::libc::memcpy(
        zOut as *mut ::core::ffi::c_void,
        zIn as *const ::core::ffi::c_void,
        nCopy as crate::__stddef_size_t_h::size_t,
    );
}
"""
        matches1 = self.find(code1)
        if len(matches1) != 1:
            print(f"FAIL test1 find: expected 1 match, got {len(matches1)}")
            return False
        fixed1 = self.fix(code1)
        if "::libc::memcpy" in fixed1:
            print("FAIL test1 fix: ::libc::memcpy still present")
            return False
        if "copy_nonoverlapping" not in fixed1:
            print("FAIL test1 fix: copy_nonoverlapping not inserted")
            return False
        # Arg order: src must appear before dst in the call
        src_pos1 = fixed1.find("zIn as *const u8")
        dst_pos1 = fixed1.find("zOut as *mut u8")
        if src_pos1 == -1 or dst_pos1 == -1:
            print(f"FAIL test1 fix: src/dst not found: {fixed1!r}")
            return False
        if src_pos1 > dst_pos1:
            print("FAIL test1 fix: src appears after dst (arg order not reversed)")
            return False
        if "nCopy as usize" not in fixed1:
            print("FAIL test1 fix: size not converted to usize")
            return False

        # ── Test 2: memmove with multi-level cast (inner type preserved) ────
        code2 = r"""
unsafe fn defrag(data: *mut u8, a: i32, b: i32, n: i32) {
    ::libc::memmove(
        data.offset(a as isize) as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        data.offset(b as isize) as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        n as crate::__stddef_size_t_h::size_t,
    );
}
"""
        matches2 = self.find(code2)
        if len(matches2) != 1:
            print(f"FAIL test2 find: expected 1 match, got {len(matches2)}")
            return False
        fixed2 = self.fix(code2)
        if "::libc::memmove" in fixed2:
            print("FAIL test2 fix: ::libc::memmove still present")
            return False
        if "::core::ptr::copy(" not in fixed2:
            print("FAIL test2 fix: ::core::ptr::copy not inserted")
            return False
        # Inner multi-level casts should be preserved as inner text
        if "::core::ffi::c_uchar" not in fixed2:
            print("FAIL test2 fix: inner uchar cast lost")
            return False
        if "n as usize" not in fixed2:
            print("FAIL test2 fix: size not converted to usize")
            return False

        # ── Test 3: memcpy with size_of expression ───────────────────────────
        code3 = r"""
unsafe fn copy_mem(pTo: *mut Mem, pFrom: *const Mem) {
    ::libc::memcpy(
        pTo as *mut ::core::ffi::c_void,
        pFrom as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Mem>() as crate::__stddef_size_t_h::size_t,
    );
}
"""
        fixed3 = self.fix(code3)
        if "::libc::memcpy" in fixed3:
            print("FAIL test3: ::libc::memcpy still present")
            return False
        if "copy_nonoverlapping" not in fixed3:
            print("FAIL test3: copy_nonoverlapping not inserted")
            return False
        if "size_of::<Mem>() as usize" not in fixed3:
            print("FAIL test3: size_of expression not converted correctly")
            return False

        # ── Test 4: no match when void cast is absent (guard check) ─────────
        code4 = r"""
unsafe fn copy_raw(dst: *mut u8, src: *const u8, n: usize) {
    ::libc::memcpy(dst as *mut u8, src as *const u8, n);
}
"""
        matches4 = self.find(code4)
        if len(matches4) != 0:
            print(f"FAIL test4: expected 0 matches (no void casts), got {len(matches4)}")
            return False

        # ── Test 5: idempotency — running fix twice produces same result ─────
        code5 = r"""
unsafe fn copy_page2(zOut: *mut u8, zIn: *const u8, nCopy: i32) {
    ::libc::memcpy(
        zOut as *mut ::core::ffi::c_void,
        zIn as *const ::core::ffi::c_void,
        nCopy as crate::__stddef_size_t_h::size_t,
    );
}
"""
        fixed5a = self.fix(code5)
        fixed5b = self.fix(fixed5a)
        if fixed5a != fixed5b:
            print("FAIL test5: fix is not idempotent")
            return False

        # ── Test 6: multiple calls in one function ───────────────────────────
        code6 = r"""
unsafe fn multi(a: *mut u8, b: *const u8, c: *mut u8, d: *const u8, n: usize) {
    ::libc::memcpy(a as *mut ::core::ffi::c_void, b as *const ::core::ffi::c_void, n as crate::__stddef_size_t_h::size_t);
    ::libc::memmove(c as *mut ::core::ffi::c_void, d as *const ::core::ffi::c_void, n as crate::__stddef_size_t_h::size_t);
}
"""
        matches6 = self.find(code6)
        if len(matches6) != 2:
            print(f"FAIL test6: expected 2 matches, got {len(matches6)}")
            return False
        fixed6 = self.fix(code6)
        if "::libc::memcpy" in fixed6 or "::libc::memmove" in fixed6:
            print("FAIL test6: libc calls still present after fix")
            return False
        if fixed6.count("copy_nonoverlapping") != 1 or fixed6.count("::core::ptr::copy(") != 1:
            print("FAIL test6: expected one of each ptr::copy variant")
            return False

        # ── Test 7: type-preserving — c_char inner type (most common case) ──
        code7a = r"""
unsafe fn copy_chars(buf: *mut ::core::ffi::c_char, src: *const ::core::ffi::c_char, n: i32) {
    ::libc::memcpy(
        buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        n as crate::__stddef_size_t_h::size_t,
    );
}
"""
        fixed7a = self.fix(code7a)
        if "::libc::memcpy" in fixed7a:
            print("FAIL test7a: ::libc::memcpy still present")
            return False
        # Must use c_char, NOT u8, because both sides have the same type
        if "*const ::core::ffi::c_char" not in fixed7a:
            print(f"FAIL test7a: *const c_char not found (got: {fixed7a!r})")
            return False
        if "*mut ::core::ffi::c_char" not in fixed7a:
            print(f"FAIL test7a: *mut c_char not found (got: {fixed7a!r})")
            return False
        # Must NOT have a redundant u8 cast
        if " as *mut u8" in fixed7a or " as *const u8" in fixed7a:
            print(f"FAIL test7a: redundant u8 cast added: {fixed7a!r}")
            return False

        # ── Test 8: type-preserving — u8_0 alias (second most common) ────────
        code8 = r"""
unsafe fn copy_u8(buf: *mut u8_0, src: *const u8_0, n: usize) {
    ::libc::memcpy(
        buf as *mut u8_0 as *mut ::core::ffi::c_void,
        src as *const u8_0 as *const ::core::ffi::c_void,
        n as crate::__stddef_size_t_h::size_t,
    );
}
"""
        fixed8 = self.fix(code8)
        if "::libc::memcpy" in fixed8:
            print("FAIL test8: ::libc::memcpy still present")
            return False
        # Both sides have u8_0 (canonical u8); result must also use u8_0
        # (the original trailing type is preserved, not remapped to canonical string)
        if "*mut u8_0" not in fixed8:
            print(f"FAIL test8: *mut u8_0 not found: {fixed8!r}")
            return False
        if "*const u8_0" not in fixed8:
            print(f"FAIL test8: *const u8_0 not found: {fixed8!r}")
            return False

        # ── Test 9: mixed byte types fall back to u8 ─────────────────────────
        # dst uses c_char (i8) and src uses c_uchar (u8) — different canonical types
        code9 = r"""
unsafe fn copy_mixed(buf: *mut ::core::ffi::c_char, src: *const ::core::ffi::c_uchar, n: usize) {
    ::libc::memcpy(
        buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        n as crate::__stddef_size_t_h::size_t,
    );
}
"""
        fixed9 = self.fix(code9)
        if "::libc::memcpy" in fixed9:
            print("FAIL test9: ::libc::memcpy still present")
            return False
        # c_char (i8) != c_uchar (u8), so must fall back to u8
        if "*mut u8" not in fixed9:
            print(f"FAIL test9: *mut u8 fallback not found: {fixed9!r}")
            return False

        # ── Test 10: multi-byte inner type falls back to u8 ───────────────────
        code10 = r"""
unsafe fn copy_ints(buf: *mut ::core::ffi::c_int, src: *const ::core::ffi::c_int, n: usize) {
    ::libc::memcpy(
        buf as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_int as *const ::core::ffi::c_void,
        n as crate::__stddef_size_t_h::size_t,
    );
}
"""
        fixed10 = self.fix(code10)
        if "::libc::memcpy" in fixed10:
            print("FAIL test10: ::libc::memcpy still present")
            return False
        # c_int is 4 bytes — cannot use as element type with byte count; must fall back to u8
        if "*mut u8" not in fixed10:
            print(f"FAIL test10: *mut u8 fallback not found for c_int: {fixed10!r}")
            return False

        # ── Test 11: wrapping_mul size expression (fts3 real-world case) ───────
        # This was the original regression: the regex used to strip `size_t` from
        # INSIDE `.wrapping_mul(nIndex as size_t)`, breaking the call structure.
        code11 = r"""
unsafe fn copy_index(dst: *mut Fts3Index, src: *const Fts3Index, nIndex: usize) {
    ::libc::memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        (::core::mem::size_of::<Fts3Index>() as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(nIndex as crate::__stddef_size_t_h::size_t),
    );
}
"""
        fixed11 = self.fix(code11)
        if "::libc::memcpy" in fixed11:
            print("FAIL test11: ::libc::memcpy still present")
            return False
        if "copy_nonoverlapping" not in fixed11:
            print("FAIL test11: copy_nonoverlapping not inserted")
            return False
        if "wrapping_mul" not in fixed11:
            print("FAIL test11: wrapping_mul expression was lost")
            return False
        if "as usize" not in fixed11:
            print("FAIL test11: size not cast to usize")
            return False
        open_parens = fixed11.count("(")
        close_parens = fixed11.count(")")
        if open_parens != close_parens:
            print(f"FAIL test11: unbalanced parens — {open_parens} vs {close_parens}")
            return False

        return True
