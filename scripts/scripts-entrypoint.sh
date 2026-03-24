# #!/bin/bash
set -e

# parent path relatively to current file
PARENT_PATH=$(dirname "$0")

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

CLAUDE_LOCAL_JSON="$HOME/.claude.json"

# Write "{}" only when the file is missing **or** empty
if [ ! -s "$CLAUDE_LOCAL_JSON" ]; then
    printf '{}\n' > "$CLAUDE_LOCAL_JSON"
fi


claude --plugin-dir /plugin
