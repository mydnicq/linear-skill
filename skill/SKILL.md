---
name: linear-skill
description: >
  Query Linear's GraphQL API from the command line. Search issues, list teams,
  create and update issues, and run arbitrary GraphQL queries. Use when the user
  mentions Linear, issues, sprints, or project management tasks that involve Linear.
compatibility: Supports macOS (Keychain), Windows (Credential Manager), and Linux (keyutils/Secret Service).
metadata:
  author: tadejstanic
  version: "0.1.0"
---

The binary is located at `./linear-skill` relative to this file. All commands below use `./linear-skill` — resolve the path relative to the directory containing this SKILL.md.

## Installation (auto)

Before first use, check if `./linear-skill` exists. If not, run:

```bash
bash ./install.sh
```

This downloads the correct binary for the current OS and architecture.

The API key is stored in the OS credential store (macOS Keychain, Windows Credential Manager, or Linux keyutils/Secret Service) — agents cannot read it.

## Setup (human only)

```bash
./linear-skill auth
```

You will be prompted to enter your Linear API key (hidden input). Get one from **Linear → Settings → API → Personal API keys**.

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

**The identifier field is not a valid IssueFilter field.** To fetch an issue by its human-readable identifier (e.g. WEB-385), split it into team key and number and filter with: `{"filter": {"number": {"eq": 385}, "team": {"key": {"eq": "WEB"}}}}`.

## Common Queries

All parameterized queries below use `--query-base64`. The base64 value encodes the full GraphQL query string including exclamation marks for non-null types.

### Current user

```bash
./linear-skill query --query '{ viewer { id name email } }'
```

### List teams

```bash
./linear-skill query --query '{ teams { nodes { id name key } } }'
```

### Issues assigned to me

```bash
./linear-skill query --query '{ viewer { assignedIssues(first: 20) { nodes { id identifier title state { name } priority } } } }'
```

### Search issues

```bash
./linear-skill query --query-base64 'cXVlcnkoJHRlcm06IFN0cmluZyEpIHsgc2VhcmNoSXNzdWVzKHRlcm06ICR0ZXJtLCBmaXJzdDogMTApIHsgbm9kZXMgeyBpZCBpZGVudGlmaWVyIHRpdGxlIHN0YXRlIHsgbmFtZSB9IH0gfSB9' --variables '{"term": "bug"}'
```

Decoded: query($term: String NON-NULL) { searchIssues(term: $term, first: 10) { nodes { id identifier title state { name } } } }

### Fetch issue by identifier

```bash
./linear-skill query --query-base64 'cXVlcnkoJGZpbHRlcjogSXNzdWVGaWx0ZXIhKSB7IGlzc3VlcyhmaWx0ZXI6ICRmaWx0ZXIsIGZpcnN0OiAxKSB7IG5vZGVzIHsgaWQgaWRlbnRpZmllciB0aXRsZSBkZXNjcmlwdGlvbiBzdGF0ZSB7IG5hbWUgfSBwcmlvcml0eSBhc3NpZ25lZSB7IG5hbWUgfSBsYWJlbHMgeyBub2RlcyB7IG5hbWUgfSB9IGNyZWF0ZWRBdCB1cGRhdGVkQXQgfSB9IH0=' --variables '{"filter": {"number": {"eq": 385}, "team": {"key": {"eq": "WEB"}}}}'
```

Decoded: query($filter: IssueFilter NON-NULL) { issues(filter: $filter, first: 1) { nodes { id identifier title description state { name } priority assignee { name } labels { nodes { name } } createdAt updatedAt } } }

### Workflow states for a team

```bash
./linear-skill query --query-base64 'cXVlcnkoJHRlYW1JZDogU3RyaW5nISkgeyB0ZWFtKGlkOiAkdGVhbUlkKSB7IHN0YXRlcyB7IG5vZGVzIHsgaWQgbmFtZSB0eXBlIH0gfSB9IH0=' --variables '{"teamId": "TEAM_ID"}'
```

Decoded: query($teamId: String NON-NULL) { team(id: $teamId) { states { nodes { id name type } } } }

### Create an issue

```bash
./linear-skill query --query-base64 'bXV0YXRpb24oJGlucHV0OiBJc3N1ZUNyZWF0ZUlucHV0ISkgeyBpc3N1ZUNyZWF0ZShpbnB1dDogJGlucHV0KSB7IHN1Y2Nlc3MgaXNzdWUgeyBpZCBpZGVudGlmaWVyIHRpdGxlIHVybCB9IH0gfQ==' --variables '{"input": {"teamId": "TEAM_ID", "title": "Issue title", "description": "Description"}}'
```

Decoded: mutation($input: IssueCreateInput NON-NULL) { issueCreate(input: $input) { success issue { id identifier title url } } }

### Update an issue

```bash
./linear-skill query --query-base64 'bXV0YXRpb24oJGlkOiBTdHJpbmchLCAkaW5wdXQ6IElzc3VlVXBkYXRlSW5wdXQhKSB7IGlzc3VlVXBkYXRlKGlkOiAkaWQsIGlucHV0OiAkaW5wdXQpIHsgc3VjY2VzcyBpc3N1ZSB7IGlkIHRpdGxlIHN0YXRlIHsgbmFtZSB9IH0gfSB9' --variables '{"id": "ISSUE_ID", "input": {"stateId": "STATE_ID"}}'
```

Decoded: mutation($id: String NON-NULL, $input: IssueUpdateInput NON-NULL) { issueUpdate(id: $id, input: $input) { success issue { id title state { name } } } }

## Self-Reflection Protocol

**After every interaction with linear-skill, you MUST follow this protocol:**

1. **Check for errors.** If any CLI call failed or returned unexpected results, analyze the root cause.
2. **Classify the error.** Determine if the cause is:
   - Shell/environment (escaping, quoting, PATH issues)
   - GraphQL (wrong field names, missing required arguments, schema changes)
   - Auth/network (expired key, rate limits, connectivity)
   - Usage (wrong flags, malformed JSON variables)
3. **Update this file.** If the error is not already documented in the **Shell Pitfalls** section above, append a new entry describing:
   - The symptom (exact error message or behavior)
   - The root cause
   - The workaround or correct approach
4. **Keep it concise.** Each entry should be 2-4 lines. Remove entries that are superseded by better solutions.

This ensures lessons learned are always available for future calls, preventing the same mistake twice.

## Reference

Full GraphQL schema: https://studio.apollographql.com/public/Linear-API/variant/current/home
