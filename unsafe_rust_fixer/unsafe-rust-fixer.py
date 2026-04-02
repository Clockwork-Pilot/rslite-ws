#!/usr/bin/env python3
"""Unsafe Rust Pattern Fixer - Main Engine

Discovers and optionally fixes named unsafe patterns in Rust code using tree-sitter.

Usage:
    unsafe-rust-fixer.py <rust-file> [--list]
    unsafe-rust-fixer.py <rust-file> --fix <pattern-name> [<pattern-name> ...]
    unsafe-rust-fixer.py --list-patterns

Examples:
    # List all detected patterns
    unsafe-rust-fixer.py main.rs

    # List available patterns
    unsafe-rust-fixer.py --list-patterns

    # Fix specific patterns
    unsafe-rust-fixer.py main.rs --fix unsafe_deref_raw_ptr unsafe_cast

    # Perform all available fixes
    unsafe-rust-fixer.py main.rs --fix all
"""

import argparse
import sys
import os
import multiprocessing
from pathlib import Path
from typing import Dict, List, Tuple, Optional, Any
import importlib.util
import fnmatch


class UnsafePatternFixer:
    """Engine for discovering and fixing unsafe patterns in Rust code."""

    def __init__(self, verbose: bool = False) -> None:
        """Initialize the fixer and discover plugins.

        Args:
            verbose: Enable verbose output
        """
        self.verbose = verbose
        self.plugins: Dict[str, Any] = {}
        self.skipped_plugins: List[str] = []
        self.expected_plugin_count: int = 0
        self._load_plugins()

    def _load_plugins(self) -> None:
        """Dynamically load all plugins from the plugins directory."""
        base_dir = Path(__file__).parent
        patterns_dir = base_dir / "patterns"

        if not patterns_dir.exists():
            if self.verbose:
                print(f"Warning: Plugins directory not found at {patterns_dir}")
            return

        # Import base class first
        base_path = base_dir / "base.py"
        if base_path.exists():
            base_spec = importlib.util.spec_from_file_location(
                "unsafe_rust_fixer.base", base_path
            )
            if base_spec and base_spec.loader:
                base_module = importlib.util.module_from_spec(base_spec)
                sys.modules["unsafe_rust_fixer.base"] = base_module
                try:
                    base_spec.loader.exec_module(base_module)
                except Exception as e:
                    if self.verbose:
                        print(f"Error loading base module: {e}")
                        import traceback
                        traceback.print_exc()
                    return
            else:
                if self.verbose:
                    print("Failed to load base module")
                return
        else:
            if self.verbose:
                print(f"Base module not found at {base_path}")
            return

        # Also register base module under patterns namespace for relative imports
        sys.modules["unsafe_rust_fixer.patterns.base"] = base_module

        # Load all plugin modules from patterns directory
        plugin_files = sorted(patterns_dir.glob("*.py"))
        self.expected_plugin_count = len([f for f in plugin_files if not f.name.startswith("_")])

        for plugin_file in plugin_files:
            if plugin_file.name.startswith("_"):
                continue

            plugin_loaded = False
            try:
                # Create proper module path for sys.modules
                module_name = f"unsafe_rust_fixer.patterns.{plugin_file.stem}"
                spec = importlib.util.spec_from_file_location(
                    module_name, plugin_file
                )
                if spec is None or spec.loader is None:
                    self.skipped_plugins.append(plugin_file.stem)
                    continue

                module = importlib.util.module_from_spec(spec)
                sys.modules[module_name] = module
                spec.loader.exec_module(module)

                # Find and instantiate plugin class
                for attr_name in dir(module):
                    if attr_name.startswith("_"):
                        continue
                    attr = getattr(module, attr_name)
                    try:
                        if (isinstance(attr, type) and
                            issubclass(attr, base_module.UnsafePatternPlugin) and
                            attr is not base_module.UnsafePatternPlugin):
                            plugin = attr()
                            self.plugins[plugin.name] = plugin
                            plugin_loaded = True
                            if self.verbose:
                                print(f"Loaded plugin: {plugin.name}")
                    except (TypeError, AttributeError):
                        pass

                if not plugin_loaded:
                    self.skipped_plugins.append(plugin_file.stem)

            except Exception as e:
                self.skipped_plugins.append(plugin_file.stem)
                if self.verbose:
                    print(f"Error loading plugin {plugin_file.name}: {e}")
                    import traceback
                    traceback.print_exc()

    def list_patterns(self) -> bool:
        """List all available patterns sorted by priority.

        Returns:
            True if a plugin count mismatch was detected, False otherwise.
        """
        if not self.plugins:
            print("No patterns available. Check that plugins are installed.")
            return False

        # Check for plugin mismatch
        has_mismatch = bool(self.skipped_plugins)
        if has_mismatch:
            print(f"⚠️  WARNING: Plugin loading issue detected!")
            print(f"   Expected {self.expected_plugin_count} plugin(s), but only {len(self.plugins)} loaded successfully.")
            print(f"   Skipped plugins ({len(self.skipped_plugins)}): {', '.join(sorted(self.skipped_plugins))}\n")

        print(f"Available unsafe patterns ({len(self.plugins)}):\n")
        # Sort by priority (descending), then by name
        sorted_names = sorted(
            self.plugins.keys(),
            key=lambda name: (-self.plugins[name].priority, name)
        )
        for name in sorted_names:
            plugin = self.plugins[name]
            print(f"  {name} (priority: {plugin.priority})")
            print(f"    {plugin.description}\n")
        return has_mismatch

    def run_tests(self) -> int:
        """Run tests for all plugins.

        Returns:
            0 if all tests pass, 1 otherwise
        """
        if not self.plugins:
            print("No plugins available to test.")
            return 1

        print(f"Running tests for {len(self.plugins)} plugin(s)...\n")

        all_passed = True
        for name in sorted(self.plugins.keys()):
            plugin = self.plugins[name]
            print(f"Testing {name}:")
            try:
                passed = plugin.test()
                if not passed:
                    all_passed = False
                    print(f"  ✗ Tests failed")
                else:
                    print(f"  ✓ All tests passed")
            except Exception as e:
                print(f"  ✗ Error running tests: {e}")
                if self.verbose:
                    import traceback
                    traceback.print_exc()
                all_passed = False
            print()

        return 0 if all_passed else 1

    def expand_glob_patterns(self, match_patterns: List[str]) -> List[str]:
        """Expand glob patterns to match available plugins.

        Args:
            match_patterns: List of pattern names or glob patterns (e.g., ['*', 'raw_*'])

        Returns:
            Expanded list of pattern names
        """
        expanded = []
        available_patterns = set(self.plugins.keys())

        for pattern in match_patterns:
            if pattern == '*':
                # Match all available patterns
                expanded.extend(sorted(available_patterns))
            else:
                # Use fnmatch for glob-style pattern matching
                matches = fnmatch.filter(available_patterns, pattern)
                if matches:
                    expanded.extend(matches)
                else:
                    # If no matches, still include the pattern (will be reported as not found later)
                    expanded.append(pattern)

        # Remove duplicates while preserving order
        seen = set()
        result = []
        for p in expanded:
            if p not in seen:
                seen.add(p)
                result.append(p)
        return result

    def find_patterns(self, rust_file: str, match_patterns: Optional[List[str]] = None) -> Dict[str, List[Tuple[int, int, str]]]:
        """Find all unsafe patterns in a Rust file.

        Processes patterns in priority order (higher priority first).

        Args:
            rust_file: Path to Rust source file
            match_patterns: Optional list of pattern names to match (if None, run all)

        Returns:
            Dict mapping pattern names to list of findings
        """
        if not os.path.exists(rust_file):
            raise FileNotFoundError(f"File not found: {rust_file}")

        with open(rust_file, 'r', encoding='utf-8') as f:
            code = f.read()

        results: Dict[str, List[Tuple[int, int, str]]] = {}

        # Expand glob patterns if specified
        if match_patterns:
            match_patterns = self.expand_glob_patterns(match_patterns)

        # Sort patterns by priority (descending)
        sorted_pattern_names = sorted(
            self.plugins.keys(),
            key=lambda name: -self.plugins[name].priority
        )

        for pattern_name in sorted_pattern_names:
            plugin = self.plugins[pattern_name]
            # Skip if match_patterns is specified and this pattern is not in it
            if match_patterns and pattern_name not in match_patterns:
                continue

            try:
                findings = plugin.find(code)
                if findings:
                    results[pattern_name] = findings
                    if self.verbose:
                        print(f"Found {len(findings)} issues with {pattern_name}")
            except Exception as e:
                if self.verbose:
                    print(f"Error running {pattern_name}: {e}")

        return results

    def _get_code_snippet(self, code: str, start_byte: int, end_byte: int, context_lines: int = 1) -> str:
        """Extract a snippet of code around a match.

        Args:
            code: Full source code
            start_byte: Start byte of match
            end_byte: End byte of match
            context_lines: Lines of context before/after match

        Returns:
            Snippet with context
        """
        lines = code.split('\n')

        # Find which line the match starts on
        byte_pos = 0
        start_line = 0
        for i, line in enumerate(lines):
            line_bytes = len(line) + 1  # +1 for newline
            if byte_pos + line_bytes > start_byte:
                start_line = i
                break
            byte_pos += line_bytes

        # Get context
        context_start = max(0, start_line - context_lines)
        context_end = min(len(lines), start_line + context_lines + 1)

        snippet_lines = lines[context_start:context_end]
        return '\n'.join(snippet_lines)

    def _get_line_number(self, code: str, byte_pos: int) -> int:
        """Get line number from byte position."""
        return code[:byte_pos].count('\n') + 1

    def report_findings(
        self, rust_file: str, findings: Dict[str, List[Tuple[int, int, str]]]
    ) -> None:
        """Print a human-readable report of findings.

        Args:
            rust_file: Path to the Rust file
            findings: Dict of pattern names to findings
        """
        if not findings:
            print(f"✓ No unsafe patterns found in {rust_file}")
            return

        total = sum(len(f) for f in findings.values())
        print(f"\n{rust_file}: Found {total} unsafe pattern(s)\n")

        for pattern_name in sorted(findings.keys()):
            plugin = self.plugins[pattern_name]
            findings_list = findings[pattern_name]
            print(f"[{pattern_name}] {plugin.description}")
            for _, _, description in findings_list:
                print(f"  - {description}")
            print()

    def report_matched_patterns(
        self, rust_file: str, findings: Dict[str, List[Tuple[int, int, str]]]
    ) -> None:
        """Print matched patterns with file, line, and snippet.

        Args:
            rust_file: Path to the Rust file
            findings: Dict of pattern names to findings
        """
        if not os.path.exists(rust_file):
            return

        with open(rust_file, 'r', encoding='utf-8') as f:
            code = f.read()

        if not findings or not self.verbose:
            return

        for pattern_name in sorted(findings.keys()):
            plugin = self.plugins[pattern_name]
            findings_list = findings[pattern_name]

            for start_byte, end_byte, description in findings_list:
                line_num = self._get_line_number(code, start_byte)
                matched_text = code[start_byte:end_byte]
                snippet = self._get_code_snippet(code, start_byte, end_byte, context_lines=0)

                print(f"{rust_file}:{line_num} [{pattern_name}]")
                print(f"  Matched: {matched_text[:80]}")
                if snippet:
                    print(f"  Code: {snippet[:100]}")
                print()

    def apply_fixes_to_patterns(
        self, rust_file: str, findings: Dict[str, List[Tuple[int, int, str]]],
        dry_run: bool = False
    ) -> bool:
        """Apply fixes to the patterns that were matched.

        Args:
            rust_file: Path to Rust source file
            findings: Dict of matched pattern names to findings
            dry_run: If True, compute fixes but do not write to disk
        """
        if not os.path.exists(rust_file):
            raise FileNotFoundError(f"File not found: {rust_file}")

        if not findings:
            print(f"No patterns to fix in {rust_file}")
            return

        with open(rust_file, 'r', encoding='utf-8') as f:
            code = f.read()

        # Apply fixes for each matched pattern
        original_code = code
        fixed_count = 0

        for pattern_name in sorted(findings.keys()):
            if pattern_name not in self.plugins:
                print(f"Warning: Pattern '{pattern_name}' not found")
                continue

            plugin = self.plugins[pattern_name]
            try:
                before_code = code
                code = plugin.fix(code)
                if code != before_code:
                    fixed_count += 1
                    if self.verbose:
                        print(f"Applied fixes for {pattern_name}")
            except Exception as e:
                print(f"Error applying fix for {pattern_name}: {e}")

        # Write back if changes were made (skip if dry-run)
        if code != original_code:
            if self.verbose:
                print(f"✓ Fixed {fixed_count} pattern group(s)")
            if dry_run:
                if self.verbose:
                    print(f"✓ Would update {rust_file} (dry-run, not written)")
            else:
                with open(rust_file, 'w', encoding='utf-8') as f:
                    f.write(code)
                if self.verbose:
                    print(f"✓ Updated {rust_file}")

        return code != original_code


# ── Parallel worker state ────────────────────────────────────────────────────
# Each worker process initializes its own UnsafePatternFixer once, then reuses
# it for every (file, pattern) task it receives from the pool.

_worker_fixer: Optional[UnsafePatternFixer] = None


def _init_worker() -> None:
    global _worker_fixer
    _worker_fixer = UnsafePatternFixer(verbose=False)


def _process_file_for_pattern(
    args: Tuple[str, str, bool, bool]
) -> Tuple[str, Dict, bool]:
    """Find and optionally fix one pattern in one file (runs in worker process)."""
    file_path, pattern_name, do_fix, dry_run = args
    try:
        assert _worker_fixer is not None
        findings = _worker_fixer.find_patterns(file_path, match_patterns=[pattern_name])
        changed = False
        if do_fix and findings:
            changed = _worker_fixer.apply_fixes_to_patterns(file_path, findings, dry_run=dry_run)
        return file_path, findings, changed
    except Exception:
        return file_path, {}, False


def main() -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Find and fix unsafe patterns in Rust code",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )

    parser.add_argument(
        "file",
        nargs="?",
        help="Rust source file or project directory to analyze (default: current directory)"
    )
    parser.add_argument(
        "--match-patterns",
        metavar="PATTERNS",
        required=False,
        help="Comma-separated list of patterns to match. Supports glob patterns: * matches all patterns, raw_* matches patterns starting with 'raw_' (e.g., unsafe_deref_raw_ptr,raw_*,*)"
    )
    parser.add_argument(
        "--fix",
        action="store_true",
        help="Apply fixes to matched patterns (requires --match-patterns)"
    )
    parser.add_argument(
        "--list-patterns",
        action="store_true",
        help="List all available patterns and exit"
    )
    parser.add_argument(
        "--test",
        action="store_true",
        help="Run self-contained tests for all plugins"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Run full find+fix flow but do not write any changes to disk"
    )
    parser.add_argument(
        "-v", "--verbose",
        action="store_true",
        help="Enable verbose output"
    )

    args = parser.parse_args()

    try:
        # Validate: --fix requires --match-patterns
        if args.fix and not args.match_patterns:
            print("Error: --fix requires --match-patterns to be specified")
            return 1

        fixer = UnsafePatternFixer(verbose=args.verbose)

        if args.list_patterns:
            has_mismatch = fixer.list_patterns()
            return 1 if has_mismatch else 0

        if args.test:
            return fixer.run_tests()

        # Parse match_patterns if specified
        match_patterns: Optional[List[str]] = None
        if args.match_patterns:
            match_patterns = [p.strip() for p in args.match_patterns.split(',')]
            if args.verbose:
                print(f"Matching patterns: {match_patterns}")

        # Without --match-patterns, show help
        if not args.match_patterns:
            parser.print_help()
            return 0

        # Determine target: file or project directory
        target = args.file or os.getcwd()

        if args.verbose:
            print(f"Target: {target}")

        # Check if target is a directory (project) or file
        if os.path.isdir(target):
            # Scan all Rust files in the directory recursively
            rust_files = sorted(Path(target).rglob("*.rs"))
            if not rust_files:
                print(f"No Rust files found in {target}")
                return 0

            if args.verbose:
                print(f"Found {len(rust_files)} Rust file(s)")

            # Patterns applied sequentially by priority; files parallelized per pattern.
            expanded = fixer.expand_glob_patterns(match_patterns) if match_patterns else list(fixer.plugins.keys())
            sorted_patterns = sorted(
                [p for p in expanded if p in fixer.plugins],
                key=lambda name: -fixer.plugins[name].priority,
            )

            total_findings: Dict[str, Dict] = {}
            dry_run_skip_count = 0
            num_workers = min(os.cpu_count() or 1, len(rust_files))

            with multiprocessing.Pool(processes=num_workers, initializer=_init_worker) as pool:
                for pattern_name in sorted_patterns:
                    if args.verbose:
                        print(f"Running pattern: {pattern_name}")
                    work = [
                        (str(f), pattern_name, args.fix, args.dry_run)
                        for f in rust_files
                    ]
                    for file_path, findings, changed in pool.map(_process_file_for_pattern, work):
                        if findings:
                            total_findings.setdefault(file_path, {}).update(findings)
                            fixer.report_matched_patterns(file_path, findings)
                        if args.dry_run and changed:
                            dry_run_skip_count += 1

            if args.dry_run:
                total_occurrences = sum(len(v) for f in total_findings.values() for v in f.values())
                print(f"Dry-run summary:")
                print(f"Occurrences matched: {total_occurrences}")
                print(f"Fixes skipped (dry-run): {dry_run_skip_count}")

            return 1 if (args.dry_run and dry_run_skip_count > 0) else 0
        else:
            # Single file mode
            findings = fixer.find_patterns(target, match_patterns=match_patterns)
            fixer.report_matched_patterns(target, findings)

            dry_run_skip_count = 0
            if args.fix and findings:
                if args.verbose:
                    print(f"\nApplying fixes to matched patterns...")
                changed = fixer.apply_fixes_to_patterns(target, findings, dry_run=args.dry_run)
                if args.dry_run and changed:
                    dry_run_skip_count += 1

            if args.dry_run:
                total_occurrences = sum(len(v) for v in findings.values())
                print(f"Dry-run summary:")
                print(f"Occurrences matched: {total_occurrences}")
                print(f"Fixes skipped (dry-run): {dry_run_skip_count}")

            return 1 if (args.dry_run and dry_run_skip_count > 0) else 0

    except FileNotFoundError as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        if args.verbose:
            import traceback
            traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())
