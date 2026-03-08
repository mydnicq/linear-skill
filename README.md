# linear-skill

An agent skill that lets you query Linear's GraphQL API directly from the command line — search issues, list teams, create and update issues, and run arbitrary GraphQL queries.

## Motivation

MCP servers are often overkill. This project demonstrates a simpler approach: a self-contained skill that gives an agent everything it needs to interact with Linear without spinning up a separate server process.

Read more: [You probably don't need MCP](https://tadejstanic.dev/blog/you-probably-dont-need-mcp/)

## Installation

**1. Download the skill**

Clone the repo and copy the skill folder into your agent skills directory:

```bash
git clone https://github.com/mydnicq/linear-skill.git
cp -r linear-skill/skill path/to/your/agent/skills/linear-skill
cd path/to/your/agent/skills/linear-skill
```

**2. Install the skill**

Start a new agent session in your project and prompt:

```
Install the linear skill
```

The agent will detect the skill, run `scripts/install.sh` automatically, and download the correct binary for your OS and architecture.

**3. Authenticate**

```bash
./linear-skill auth
```

You will be prompted for your Linear API key (hidden input). Get one from **Linear → Settings → API → Personal API keys**.

The key is stored in your OS credential store (macOS Keychain, Windows Credential Manager, or Linux keyutils/Secret Service) and scoped to the directory — different projects can use different Linear workspaces.

## Usage

Once installed, the agent will automatically use the skill when you mention Linear, issues, sprints, or project management tasks. The skill instructs the agent to:

- Search and filter issues
- List teams and projects
- Create and update issues
- Run arbitrary GraphQL queries against the Linear API

## Platform Support

| OS      | Architecture       |
|---------|--------------------|
| macOS   | arm64, amd64       |
| Linux   | amd64              |
| Windows | amd64              |

## TODO

- [ ] Self-update logic
- [ ] Research options for built-in semantic search of Linear's GraphQL schema
- [ ] Distribute the skill via npm

## License

MIT — see [LICENSE](LICENSE)
