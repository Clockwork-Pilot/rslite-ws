#!/usr/bin/env python3
"""
Git proxy wrapper — blocks destructive subcommands inside Docker.
Installed as /usr/local/bin/git (takes priority over /usr/bin/git in PATH).
Real binary is called directly without sudo.
"""
import re
import sys
import os

REAL_BINARY_DIR = "/usr/bin"

CONFIG = {
    "git": {
        "denied_subcommands": {"commit", "push", "rebase", "reset", "clean", "gc"},
        "denied_patterns":    [r"--force(?:-with-lease)?", r"-f\b"],
    },
    "gh": {
        "denied_subcommands": {"repo", "release", "secret", "auth"},
        "denied_patterns":    [],
    },
}


def main() -> None:
    called_as = os.path.basename(sys.argv[0])
    args = sys.argv[1:]
    subcommand = args[0] if args else ""

    rule = CONFIG.get(called_as)
    if rule:
        if subcommand in rule["denied_subcommands"]:
            print(f"[proxy_wrapper] blocked: '{called_as} {subcommand}' is not allowed in this container.",
                  file=sys.stderr)
            sys.exit(1)

        args_str = " ".join(args)
        for pattern in rule["denied_patterns"]:
            if re.search(pattern, args_str):
                print(f"[proxy_wrapper] blocked: forbidden flag pattern '{pattern}'.",
                      file=sys.stderr)
                sys.exit(1)

    real_binary = os.path.join(REAL_BINARY_DIR, called_as)
    os.execv(real_binary, [real_binary] + args)


if __name__ == "__main__":
    main()
