---
name: release
description: Release a new version of the linear-skill project. Bumps version in skill/SKILL.md metadata and Cargo.toml, commits, and creates a git tag matching the GitHub Actions release workflow trigger (v*). Use when the user wants to cut a release, bump the version, or publish a new version of linear-skill.
---

## Workflow

1. **Ask the user** for the new semver version (e.g. `1.2.3`). Do not guess or infer it.

2. **Update `Cargo.toml`** — change the `version` field under `[package]`:
   ```toml
   version = "X.Y.Z"
   ```

3. **Update `skill/SKILL.md`** — change the `version` field in the YAML frontmatter:
   ```yaml
   version: 'X.Y.Z'
   ```

4. **Commit** both files:
   ```bash
   git add Cargo.toml skill/SKILL.md
   git commit -m "chore: release vX.Y.Z"
   ```

5. **Tag** the commit with the `v` prefix (required by `.github/workflows/release.yml` which triggers on `v*` tags):
   ```bash
   git tag vX.Y.Z
   ```

6. **Ask the user** if they want to push the commit and tag to origin before running:
   ```bash
   git push origin main --tags
   ```

## Notes

- The GitHub Actions release workflow triggers on tags matching `v*` — the `v` prefix is mandatory.
- Pushing the tag triggers cross-platform builds (Linux amd64, macOS arm64/amd64, Windows amd64) and creates a GitHub Release with auto-generated release notes.
- Always confirm the version with the user before making any changes.
