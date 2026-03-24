#!/bin/bash
set -e

CLAUDE_LOCAL_JSON="$HOME/.claude.json"

PARENT_PATH=$(dirname "$0")
source $PARENT_PATH/filter_context.sh
