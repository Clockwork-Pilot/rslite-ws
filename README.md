# Get
``` bash
# separately clone repo
git clone git@github.com:Clockwork-Pilot/rslite-ws.git

# fetch / update submodules: ra_ap_shell, claude-plugin
git submodule update --init --recursive
```

# Build Docker:

```bash
docker build -t rslite-ws .
```

# Run in docker


```bash
# ensure venv is created
./run-docker-work-on-sqlite.sh "~/create-venv-docker.sh"

# run claude code using defaults
./run-docker-work-on-sqlite.sh

# or run command explicitely. Note: we specify destination mounted paths already.
./run-docker-work-on-sqlite.sh claude --dangerously-skip-permissions --model claude-opus-4-6 --plugin-dir /plugin

# run bash
./run-docker-work-on-sqlite.sh bash

# test
./run-docker-work-on-sqlite.sh ./build_all.sh
```

# Notes on using dev loop

## Load y2 plugin and re-render spec
Our dev loop highly depends on validating features' constraints of our `spec.k.json`.
Since we don't have any tools for making changes in verified constrains, and agent is prohibited from making changes
in constraints that once failed. Sometimes we need to make changes in constraints, so we use hacky way - actually
manually edit spec file just in text editor. So usually after this it looses its read-only attrs which needs to be restored.
We just instruct agent to `load y2 plugin and re-render spec` and it restores read-only attrs.


# Restricting claude code agent

For some reason we didn't see permissions work when we specify `--dangerously-skip-permissions` flag, so we use own permissions tricks.

See `docker-scripts/proxy_wrapper.py` approach and related 
config `docker-scripts/work-on-sqlite/proxy_wrapper_config.json`

## Standard Claude code permissions model _won't_ _work_ for us

When used in combination with `--dangerously-skip-permissions` flag, it wasn't working as expected.
So this part is just a memory.
It is corresponding to `~/.claude/settings.json` inside docker container.
Set permisisons manually in file:
`docker-claude-artifacts-c2rust-patterns/.credentials/settings.json`: 

```json
{
  "permissions": {
    "deny": [
      "Bash(git commit:*)",
      "Bash(git push:*)",
      "Bash(git config:*)",
      "Bash(git log:*)",
      "Bash(gh:*)",
      "Agent(Explore)",
      "Write(/workspace/*)",
      "Edit(/workspace/*)"
    ]
  }
}
```
