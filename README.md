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

## Our way restricting claude code agent

For some reason we didn't see permissions work when we specify `--dangerously-skip-permissions` flag, so we use own permissions tricks.

See `docker-scripts/proxy_wrapper.py` approach and related 
config `docker-scripts/work-on-sqlite/proxy_wrapper_config.json`

## Standard Claude code permissions model, within Docker

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

