#!/usr/bin/env bash
set -euo pipefail

# If no arguments were given, fall back to an interactive shell
if [ "$#" -eq 0 ]; then
    exec /bin/bash
else
    # Execute the command exactly as it was passed
    exec "$@"
fi
