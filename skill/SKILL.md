---
name: linear-skill
description: Query Linear's GraphQL API from the command line. Search issues, list teams, create and update issues, and run arbitrary GraphQL queries. Use when the user mentions Linear, issues, sprints, or project management tasks that involve Linear.
compatibility: Supports macOS (Keychain), Windows (Credential Manager), and Linux (keyutils/Secret Service).
metadata:
  author: Tadej Stanic
  version: '0.2.0'
  source: https://github.com/mydnicq/linear-skill
  license: MIT
---

The binary is located at `./linear-skill` relative to this file. All commands below use `./linear-skill` — resolve the path relative to the directory containing this SKILL.md.

## Installation (auto)

Before first use, check if `./linear-skill` exists. If not, run:

```bash
bash ./install.sh
```

This downloads the correct binary for the current OS and architecture.

The API key is stored in the OS credential store (macOS Keychain, Windows Credential Manager, or Linux keyutils/Secret Service) — agents cannot read it. Keys are scoped to the directory where `auth` is run, so different projects can use different Linear workspaces.

## Setup (human only)

Run `auth` **from the directory containing the `linear-skill` binary**:

```bash
cd /path/to/linear-skill/directory
./linear-skill auth
```

You will be prompted to enter your Linear API key (hidden input). Get one from **Linear → Settings → API → Personal API keys**. The key is stored for the current directory only — repeat for each project that needs a different key.

## Usage

```bash
# Plain text query (only for queries without exclamation marks)
./linear-skill query --query '<graphql>' [--variables '<json>']

# Base64-encoded query (recommended — always shell-safe)
./linear-skill query --query-base64 '<base64>' [--variables '<json>']
```

Output is raw JSON on stdout. Errors go to stderr.

## Shell Pitfalls

**Always use `--query-base64` for parameterized queries.** GraphQL non-null types require an exclamation mark suffix which shells escape. Base64 encoding eliminates this problem entirely. Compute the base64 string yourself and pass it via `--query-base64`.

Use plain `--query` only for simple queries that contain no exclamation marks (e.g. `{ viewer { id name } }`).

## Memory

**Before writing any GraphQL query, you MUST read [references/memory/index.md](references/memory/index.md).** It contains working, tested queries from past sessions. If a matching query exists, use it directly — do not reconstruct it from scratch.

## Self-Reflection Protocol

**After every interaction with linear-skill, you MUST follow this protocol.**

**MANDATORY FIRST STEP:** Before doing anything else in this section, output the following text verbatim to the user:

> "Running Self-Reflection Protocol... [linear-skill]"

Then work through each checkpoint below. **Do not skip a checkpoint — explicitly decide YES or NO for each one, and take the required action before moving to the next.**

### Checkpoint 1 — New query pattern

Was a GraphQL query used that is **not** already listed in the Common Queries table?

- **YES →** You MUST complete **both** sub-actions before finishing:
  1. **Create** `references/memory/<slug>.md` using the same structure as `current_user.md` (title, one-line description, decoded GraphQL block, bash command).
  2. **Edit `references/memory/index.md`** to add a row to the index table linking to the new reference file.
  - Do not say "I'll note it" or defer — use the Write and Edit tools now.
- **NO →** Write "No new query patterns. [linear-skill]" and continue.

### Checkpoint 2 — Errors

Did any CLI call fail or return unexpected results?

- **YES →** For each error, decide: was it a dead-end that led to a working query now documented in Checkpoint 1?
  - **If yes** → skip the pitfall entry. The Common Queries reference is the correct fix; a pitfall warning is redundant.
  - **If no** → classify the root cause (shell/environment, GraphQL schema, auth/network, or usage), then **immediately** append a new entry to the **Shell Pitfalls** section (2–4 lines: symptom, root cause, workaround). Remove any entry the new one supersedes.
- **NO →** Write "No errors. [linear-skill]" and continue.

### Checkpoint 3 — Done

Confirm: "Self-Reflection Protocol complete. [linear-skill]"

## Reference

Full GraphQL schema: https://studio.apollographql.com/public/Linear-API/variant/current/home
