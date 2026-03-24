# #!/bin/bash
set -em

# parent path relatively to current file
PARENT_PATH=$(dirname "$0")

source $PARENT_PATH/common.sh

# Write static Claude permissions config
mkdir -p ~/.claude
cat > ~/.claude/settings.local.json << 'EOF'
{
  "permissions": {
    "deny": [
      "Bash(rm*)",
      "Bash(sudo*)"
    ],
    "allow": [
      "Read(*)",
      "Write(*)"
    ]
  }
}
EOF

# Write "{}" only when the file is missing **or** empty
if [ ! -s "$CLAUDE_LOCAL_JSON" ]; then
    printf '{}\n' > "$CLAUDE_LOCAL_JSON"
fi

# Pre-seed context jsons
# Code to be added by Yurii

claude --plugin-dir /plugin
