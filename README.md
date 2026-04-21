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

## Docker artifacts folder

The `docker-files/` directory is automatically created when running the Docker image via `run-docker-workspace.sh`. This folder contains persistent artifacts from the container:

- `.cargo/` — Rust package manager cache and registry data
- `.credentials/` — Claude Code credentials, plugins, and authentication tokens
- `.local/` — Local user data and configuration
- `.claude.local.json` — Local Claude Code settings and session state
- `venv/` — Python virtual environment with installed packages

These artifacts persist between container runs, eliminating the need to re-download packages and re-authenticate on subsequent executions. This folder should typically be added to `.gitignore` as it contains machine-specific state and credentials.

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
3. **Perform one-time Docker setup** before running workflows — manually execute `./run-docker-workspace.sh` locally with your `PROJECT_ROOT` set to initialize the Docker environment. This one-time setup ensures that when workflows execute on the self-hosted runner, the Docker container is already properly configured and ready to work.
4. **Open an issue** in your fork describing the feature you want added. The agent follows the `features_and_constraints` skill: it patches `spec.k.json` with a new feature whose constraints check added feature; verifies each constraint first FAILS on the current code (Zero-State Rule); then implements code until every constraint PASSES via `check_spec_constraints.py`. Optionally prefix the body with YAML frontmatter:
   ```
   ---
   timeout: 20                  # optional, minutes, default 10
   model: claude-opus-4-6       # optional, default claude-haiku-4-5
   merge_into_upstream: false   # optional, default false — see note below
   ---
   <describe feature and its constraints>
   ```

   `merge_into_upstream`: when `false` (default) the agent opens a PR inside your fork; when `true` it opens a PR against the upstream (parent) repo instead. To use `true` you need:
   - a **Personal Access Token** on your user account — *classic PAT* with `public_repo` scope if you are **not** a collaborator on upstream (the typical fork contributor case), or *fine-grained PAT* with `Pull requests: write` on the upstream repo if you are a collaborator there;
   - a GitHub **environment** named `upstream-pr` in your fork (created at `https://github.com/<you>/<fork>/settings/environments`), restricted to the `main` deployment branch;
   - the PAT stored as environment secret **`UPSTREAM_PR_TOKEN`** inside that environment (not as a repo-level secret).

   Step-by-step walkthrough (including why classic vs fine-grained): see [Opening PRs against the upstream repo](#opening-prs-against-the-upstream-repo-optional) below.
   See `claude-plugin/skills/features_and_constraints/SKILL.md` for the full contract (ConstraintBash schema, `$PROJECT_ROOT`, Exit-Code Rule, Zero-State Rule, unverified-constraint blocking).

   **Note:** feature constraints are executable checks, not docstrings — they are the only thing standing between the agent and a lazy implementation. Write them to probe real payloads, exit codes, and side effects so they cannot be satisfied by stubs, hardcoded fixtures, or `echo "success"`. A constraint that the agent can shortcut is worse than no constraint: it gives false confidence. Design each one to make cheating harder than actually implementing the feature.
5. **Click the `agent-run` label** on the issue. `issue-trigger.yml` removes the label (so it's re-triggerable), verifies an online runner is labeled with your username, and dispatches `coding-agent.yml`.
6. The runner checks out your fork in isolation, runs Claude against the issue in a docker container, and — on success — asks Claude to commit, runs constraint checks, pushes an `agent/<issue>-<slug>` branch, opens a PR `[AGENT] <issue title>`, and comments the constraints report on the issue.
7. Re-applying the label on the same issue resumes the existing agent branch rather than starting over.

## Opening PRs against the upstream repo (optional)

By default the agent opens a PR inside your fork. To have it open a PR against the upstream (parent) repo instead, set `merge_into_upstream: true` in the issue frontmatter.

### Why a token is needed here but not in the browser

When *you* click "Create pull request" in the GitHub web UI, the request goes out as **your user account** — an identity that, by default, can open a PR from any fork you push to against any public upstream repo. You never think about a token because the browser session is your token.

A GitHub Actions workflow runs as a **different identity**. The auto-minted `secrets.GITHUB_TOKEN` is an installation token for the GitHub Actions GitHub App, issued as `github-actions[bot]` and scoped to exactly one repository — the repo where the workflow is defined (your fork). It has no presence on the upstream repo and cannot create PRs there, no matter what `permissions:` the workflow declares. That is why a same-repo PR works out of the box, and a cross-repo PR returns `Resource not accessible by integration (createPullRequest)`.

A Personal Access Token stored as `UPSTREAM_PR_TOKEN` solves this by giving the workflow a **user-scoped** identity for the one API call that needs it. The PAT acts as you, so the resulting PR is attributed to your user account — the same outcome as clicking the button in the UI, just initiated from CI.

Two PAT flavors exist — **classic** and **fine-grained** — and the choice is not cosmetic:
- **Classic PAT** (with `public_repo` or `repo` scope) acts with your full user permissions. Any authenticated user can open a PR from their fork to a public upstream, so a classic PAT works even when you have no write access on upstream. This is the right pick for the typical "contributor on a fork" case.
- **Fine-grained PAT** is repo-scoped and requires `Pull requests: write` on the *target* repo (upstream). GitHub's creation form only lets you grant write permissions on repos where you already have write access, so a fine-grained PAT only works if you are a collaborator/maintainer on upstream. Selecting just the fork is **not** enough — the `POST /repos/{upstream}/pulls` check runs against the upstream repo.

### One-time setup

`UPSTREAM_PR_TOKEN` is **the value of a Personal Access Token** that you generate on your user account. GitHub does not issue this token automatically — you create it in your user settings, copy the string, and paste it into your fork as an environment secret named `UPSTREAM_PR_TOKEN`. The three steps below do exactly that.

1. **Generate the token value** (this string will become `UPSTREAM_PR_TOKEN`).

   Pick the token type that matches your access level on upstream:

   #### Option A — Classic PAT (typical case: you are NOT a collaborator on upstream)

   This is the equivalent of what happens when you click "Create pull request" in the browser: the token carries your full user identity, and any authenticated user can open a PR from their fork to a public upstream. No upstream write access is required.

   Sign in as the user whose fork runs the workflow, then open <https://github.com/settings/tokens/new> and fill in:

   | Field | Value |
   |---|---|
   | *Note* | e.g. `rslite agent upstream PRs` |
   | *Expiration* | 90 days (shortest practical — calendar the rotation) |
   | *Select scopes* | `public_repo` (if upstream is public) **or** `repo` (if upstream is private) |

   Click **Generate token**. GitHub shows the `ghp_…` string **once** — copy it now.

   #### Option B — Fine-grained PAT (only if you have write access on upstream)

   Tighter scope, but only usable when you are a collaborator/maintainer on upstream, because `POST /repos/{upstream}/pulls` checks the token's permissions against the upstream repo — not the fork — and you cannot grant `Pull requests: write` on a repo you don't already have write access to. Selecting only the fork does **not** work.

   Open <https://github.com/settings/personal-access-tokens/new> and fill in:

   | Field | Value |
   |---|---|
   | *Token name* | e.g. `rslite agent upstream PRs` |
   | *Expiration* | 90 days |
   | *Resource owner* | your user account |
   | *Repository access* | *Only select repositories* → pick the **upstream** repo (e.g. `Clockwork-Pilot/rslite`) |
   | *Repository permissions* → `Pull requests` | **Read and write** |
   | *Repository permissions* → `Contents` | **Read-only** |

   Click **Generate token**. Copy the `github_pat_…` string — it's shown only once.

   The copied string (from A or B) is the value you will paste in step 3.

2. **Create a GitHub environment in your fork** at `https://github.com/<you>/<fork>/settings/environments` → *New environment*:
   - Name: `upstream-pr`.
   - *Deployment branches and tags* → *Selected branches and tags* → add rule for `main` only. This prevents a PR that edits the workflow file from exfiltrating the token.
   - (Optional) *Required reviewers*: add yourself if you want manual approval per run.

3. **Store the token as an environment secret** (scoped to `upstream-pr`, not as a repo-level secret):
   - Inside the `upstream-pr` environment → *Environment secrets* → *Add environment secret*.
   - *Name*: `UPSTREAM_PR_TOKEN` (exactly this — the workflow looks it up by name).
   - *Value*: paste the token string you copied in step 1 (`ghp_…` for a classic PAT, or `github_pat_…` for a fine-grained PAT).
   - Click *Add secret*.

4. **Verify**: open an issue in your fork with `merge_into_upstream: true` in the frontmatter, apply the `agent-run` label, and check that the resulting PR appears on the upstream repo, authored by your user account.

If you lose the token string before saving it in step 3, you cannot recover it — regenerate a new one (same form) and use the new value. Classic PATs can be revoked at <https://github.com/settings/tokens>; fine-grained PATs at <https://github.com/settings/tokens?type=beta>.

### Why an environment (not a repo secret)?

A repo-level secret is readable by every workflow and every branch in your fork — on a public fork, that is a soft credential-leak surface. An environment secret is loaded only into jobs that explicitly declare `environment: upstream-pr`, and the branch restriction ensures that a malicious PR editing the workflow file cannot reach the token. Only the tiny cross-repo-PR job opts into the environment, so the PAT never enters the self-hosted runner where the Claude agent executes.

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
