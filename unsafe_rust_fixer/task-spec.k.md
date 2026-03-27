# Specification

## Overview

Specification description

## Table of Contents

- [Overview](#overview)
- [Features](#features)
    - [Feature: dry_run_option](#dry_run_option)
      - [constraint_dry_run_no_writes](#constraint_dry_run_no_writes)
      - [constraint_dry_run_summary_output](#constraint_dry_run_summary_output)
    - [Feature: fix_all_patterns_builds_idempotent](#fix_all_patterns_builds_idempotent)
      - [constraint_fix_builds_and_idempotent](#constraint_fix_builds_and_idempotent)
    - [Feature: list_patterns_plugin_count_warning](#list_patterns_plugin_count_warning)
      - [constraint_folder_count_matches_loaded_count](#constraint_folder_count_matches_loaded_count)
    - [Feature: verbose_mode](#verbose_mode)
      - [constraint_no_verbose_output_without_flag](#constraint_no_verbose_output_without_flag)

## Features

### Feature: dry_run_option
**Add --dry-run flag to unsafe-rust-fixer.py: full flow without writing files**

**Goals:**
- unsafe-rust-fixer.py must accept a --dry-run flag.
- With --dry-run, the tool runs the full find+fix flow (AST analysis, pattern matching, fix computation) but does not write any changes to disk.
- Output should indicate what would be changed, identical to the normal --fix run, but files remain unmodified.
- In --dry-run mode, output must be clear and concise: exactly 3 short summary lines showing pattern occurrences matched and detected fixes skipped (not written).

#### constraint_dry_run_no_writes
**Description:** Behavioral+Environmental: --dry-run with --fix must not write files; md5sum of tmp file must be unchanged after run
**Command:** `tmp=$(mktemp /tmp/test_dry_run_XXXXXX.rs); printf 'unsafe fn foo(p: *mut Foo) {\n  let x = (*p).field;\n  let y = (*p).field;\n  let z = (*p).field;\n}' > "$tmp"; before=$(md5sum "$tmp" | awk '{print $1}'); python /unsafe_rust_fixer/unsafe-rust-fixer.py --match-patterns=raw_ptr_deref_field_chain --fix --dry-run "$tmp"; rc=$?; after=$(md5sum "$tmp" | awk '{print $1}'); rm -f "$tmp"; [ "$before" = "$after" ] || { echo "FAIL: --dry-run modified the file"; exit 1; }; [ $rc -eq 1 ] || { echo "FAIL: --dry-run must exit 1 when changes would be made (got exit $rc)"; exit 1; }`

#### constraint_dry_run_summary_output
**Description:** Behavioral: --dry-run output must include parseable summary line 'Fixes skipped (dry-run): N'; on a no-match file count must be 0
**Command:** `tmp=$(mktemp /tmp/test_dry_run_XXXXXX.rs); printf 'fn main() {}' > "$tmp"; output=$(python /unsafe_rust_fixer/unsafe-rust-fixer.py --match-patterns=raw_ptr_deref_field_chain --fix --dry-run "$tmp" 2>&1); rm -f "$tmp"; skipped=$(echo \"$output\" | grep -oP '(?<=Fixes skipped \(dry-run\): )\d+') || { echo "FAIL: parse error - expected 'Fixes skipped (dry-run): N' in output"; exit 1; }; [ -n "$skipped" ] || { echo "FAIL: parse error - could not parse skipped count"; exit 1; }; [ "$skipped" -eq 0 ] || { echo "FAIL: expected 0 skipped fixes on no-match file, got $skipped"; exit 1; }`

### Feature: fix_all_patterns_builds_idempotent
**All pattern fixes build cleanly and are idempotent: second dry-run pass shows 0 occurrences**

**Goals:**
- unsafe-rust-fixer.py is the sole allowed mechanism to patch Rust source files. All transformations must go through it.
- The raw_ptr_deref_field_chain plugin must implement sophisticated static analysis: full AST traversal via tree-sitter, data-flow analysis (pointer aliasing, reassignment, escape), scope-aware block selection, and mutability inference.
- Pattern matching must be invasive and high-value: only trigger when 3+ repeated (*ptr).field dereferences exist in the same lexical scope, and always produce a real safe-reference binding that replaces ALL occurrences in that scope.
- Fixes must be logically equivalent: no logic may be removed without being replaced by semantically identical safe Rust code. The hoisted reference binding must preserve mutability, lifetime scope, and field access chains exactly.
- After applying all pattern fixes to src/ via --match-patterns=* --fix, the entire codebase must compile and all tests must pass via ./build_all.sh.
- All patterns must be idempotent: after the initial fix pass, a second pass with --dry-run must report Occurrences matched: 0 and Fixes skipped (dry-run): 0 — no residual matches remain.
- Target the most advanced unsafe patterns: nested field chains ((*(*p).inner).field), double derefs, multi-pointer blocks, compound assignments — all must be handled without corruption.

#### constraint_fix_builds_and_idempotent
**Description:** Behavioral+Environmental: reset src, apply all pattern fixes, build+test must pass, then dry-run must show Occurrences matched: 0 and Fixes skipped (dry-run): 0
**Command:** `cd $WORKSPACE_ROOT && git checkout -- src/ && python /unsafe_rust_fixer/unsafe-rust-fixer.py --match-patterns='*' --fix src/ && ./build_all.sh && python /unsafe_rust_fixer/unsafe-rust-fixer.py --match-patterns='*' --fix --dry-run src/   | tee /tmp/dry_run_idempotent_check.txt; grep -q 'Occurrences matched: 0' /tmp/dry_run_idempotent_check.txt   || { echo 'FAIL: expected Occurrences matched: 0 after fix'; exit 1; }; grep -q 'Fixes skipped (dry-run): 0' /tmp/dry_run_idempotent_check.txt   || { echo 'FAIL: expected Fixes skipped (dry-run): 0 after fix'; exit 1; }`

### Feature: list_patterns_plugin_count_warning
**--list-patterns warns when folder plugin count != loaded plugin count**

**Goals:**
- When --list-patterns is invoked, detect if the number of .py plugin files in the patterns directory (excluding underscore-prefixed files) differs from the number of successfully loaded plugins.
- Show a visible warning in --list-patterns output when a mismatch is detected.
- Exit with a non-zero code when mismatch is detected so CI pipelines can catch silent plugin loading failures automatically.

#### constraint_folder_count_matches_loaded_count
**Description:** Behavioral: --list-patterns loaded plugin count must equal the number of non-underscore .py files in the patterns folder; a mismatch means a plugin failed to load silently and the warning feature should fire
**Command:** `folder_count=$(ls /unsafe_rust_fixer/patterns/*.py 2>/dev/null | grep -cv __); output=$(python /unsafe_rust_fixer/unsafe-rust-fixer.py --list-patterns 2>&1); listed_count=$(echo "$output" | grep -oP "(?<=Available unsafe patterns \\()[0-9]+(?=\\))"); [ -n "$listed_count" ] || { echo "FAIL: could not parse loaded plugin count from --list-patterns output"; exit 1; }; [ "$folder_count" -eq "$listed_count" ] || { echo "FAIL: patterns folder has $folder_count plugin file(s) but --list-patterns shows $listed_count loaded"; exit 1; }`

### Feature: verbose_mode
**--verbose is opt-in; pattern match details and fix messages are suppressed by default**

**Goals:**
- --verbose flag is off by default; detailed output (pattern match details, fix messages) must not appear unless --verbose is passed.
- Without --verbose: only the dry-run summary lines are printed.
- With --verbose: full pattern match details (file:line, Matched block, Code snippet) and per-file fix messages (Fixed N group(s), Would update, Updated) are shown.

#### constraint_no_verbose_output_without_flag
**Description:** Behavioral: without --verbose, Matched/Fixed/Would-update lines must not appear; dry-run summary must still be present
**Command:** `tmp=$(mktemp /tmp/test_verbose_XXXXXX.rs); printf 'unsafe fn foo(p: *mut Foo) {\n  let x = (*p).field;\n  let y = (*p).field;\n  let z = (*p).field;\n}' > "$tmp"; output=$(python /unsafe_rust_fixer/unsafe-rust-fixer.py --match-patterns=raw_ptr_deref_field_chain --fix --dry-run "$tmp" 2>&1); rm -f "$tmp"; echo "$output" | grep -qF 'Matched:' && { echo 'FAIL: verbose output Matched: printed without --verbose'; exit 1; }; echo "$output" | grep -qF 'Fixed' && { echo 'FAIL: verbose output Fixed printed without --verbose'; exit 1; }; echo "$output" | grep -qF 'Would update' && { echo 'FAIL: verbose output Would update printed without --verbose'; exit 1; }; echo "$output" | grep -qF 'Fixes skipped (dry-run):' || { echo 'FAIL: dry-run summary line missing'; exit 1; }`