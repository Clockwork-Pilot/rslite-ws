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

`run-docker-workspace.sh` mounts the host repo at `/workspace` inside the container. You must export `PROJECT_ROOT` (absolute host path); the script exits with an error if it's unset.

```bash
export PROJECT_ROOT=/abs/path/to/repo

# install claude code
./run-docker-workspace.sh "curl -fsSL https://claude.ai/install.sh | bash"

# run claude code using defaults
./run-docker-workspace.sh

# or run command explicitly. Note: destination mounted paths are already set.
./run-docker-workspace.sh claude --dangerously-skip-permissions --model claude-opus-4-6 --plugin-dir /plugin

# run bash
./run-docker-workspace.sh bash

# test
./run-docker-workspace.sh make c-tcl-tests
```

# Notes on using dev loop

## Load y2 plugin and re-render spec
Our dev loop highly depends on validating features' constraints of our `spec.k.json`.
Since we don't have any tools for making changes in verified constrains, and agent is prohibited from making changes
in constraints that once failed. Sometimes we need to make changes in constraints, so we use hacky way - actually
manually edit spec file just in text editor. So usually after this it looses its read-only attrs which needs to be restored.
We just instruct agent to `load y2 plugin and re-render spec` and it restores read-only attrs.

# Issue-driven coding agent (self-hosted runner)

You can wire your fork so that applying an `agent-run` label to any GitHub issue kicks off the coding agent against a dedicated branch, opens a PR, and posts a constraints report back as an issue comment.

Flow overview:

1. **Fork `rslite`** under your own GitHub account — or use any other repo you own. The agent flow, label triggers, PR creation, and constraints-driven contract are all repo-agnostic; only `spec.k.json` and the `features_and_constraints` skill tie them to a project that opts into the constraint-driven loop. For a non-`rslite` repo, copy the workflow files (`.github/workflows/coding-agent.yml`, `issue-trigger.yml`, and optionally `check-constraints.yml`) into it and pass `-e runner_repo=<github-username>/<your-repo>` when running the ansible playbook.
2. **Install a local self-hosted runner** against your fork — see [`ansible/README.md`](ansible/README.md) for the one-command ansible playbook. The runner registers with your GitHub login as a label.
3. **Open an issue** in your fork describing the feature you want added. The agent follows the `features_and_constraints` skill: it patches `spec.k.json` with a new feature whose constraints check added feature; verifies each constraint first FAILS on the current code (Zero-State Rule); then implements code until every constraint PASSES via `check_spec_constraints.py`. Optionally prefix the body with YAML frontmatter:
   ```
   ---
   timeout: 20              # optional, minutes, default 10
   model: claude-opus-4-6   # optional, default claude-haiku-4-5
   ---
   <describe feature and its constraints>
   ```
   See `claude-plugin/skills/features_and_constraints/SKILL.md` for the full contract (ConstraintBash schema, `$PROJECT_ROOT`, Exit-Code Rule, Zero-State Rule, unverified-constraint blocking).

   **Note:** feature constraints are executable checks, not docstrings — they are the only thing standing between the agent and a lazy implementation. Write them to probe real payloads, exit codes, and side effects so they cannot be satisfied by stubs, hardcoded fixtures, or `echo "success"`. A constraint that the agent can shortcut is worse than no constraint: it gives false confidence. Design each one to make cheating harder than actually implementing the feature.
4. **Click the `agent-run` label** on the issue. `issue-trigger.yml` removes the label (so it's re-triggerable), verifies an online runner is labeled with your username, and dispatches `coding-agent.yml`.
5. The runner checks out your fork in isolation, runs Claude against the issue in a docker container, and — on success — asks Claude to commit, runs constraint checks, pushes an `agent/<issue>-<slug>` branch, opens a PR `[AGENT] <issue title>`, and comments the constraints report on the issue.
6. Re-applying the label on the same issue resumes the existing agent branch rather than starting over.

# Restricting claude code agent

For some reason we didn't see permissions work when we specify `--dangerously-skip-permissions` flag, so we use own permissions tricks.

See `docker-scripts/proxy_wrapper.py` approach and related 
config `docker-scripts/proxy_wrapper_config.json`

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
