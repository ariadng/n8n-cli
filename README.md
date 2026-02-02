# n8n CLI

A powerful command-line interface for managing n8n workflows, with AI-assisted development via Claude Code.

## Features

- **Workflow Management** - Create, edit, validate, and deploy n8n workflows
- **Node Operations** - Add, remove, update, and move nodes within workflows
- **Execution Control** - Run workflows, monitor executions, retry failures
- **Claude Code Integration** - AI-assisted workflow development with natural language

## Installation

### Homebrew (macOS)

```bash
brew tap ariadng/tap
brew install n8n-cli
```

### From Source

```bash
git clone https://github.com/ariadng/n8n-cli.git
cd n8n-cli
cargo install --path .
```

## Quick Start

### 1. Configure

```bash
export N8N_BASE_URL="https://your-n8n-instance.com"
export N8N_API_KEY="your-api-key"
```

### 2. Verify Connection

```bash
n8n health check
```

### 3. List Workflows

```bash
n8n workflows list
```

## Claude Code Integration

The n8n CLI includes a Claude Code skill for AI-assisted workflow development. Install it with:

```bash
n8n install-claude-skill
```

Then in Claude Code, you can:

```
/n8n create a webhook workflow that sends Slack notifications

/n8n validate my-workflow.json

/n8n debug why workflow wf_abc123 is failing
```

Or simply ask Claude naturally:

> "Help me create an n8n workflow that syncs data from an API to PostgreSQL every hour"

The skill understands the n8n CLI commands and workflow JSON structure, helping you design, validate, and debug workflows using natural language.

[Learn more about the Claude Skill](docs/guides/claude-skill.md)

## Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `workflows` | `wf` | Manage workflows, nodes, and connections |
| `executions` | `exec` | Monitor and manage executions |
| `credentials` | `cred` | Manage credentials |
| `tags` | | Organize workflows with tags |
| `health` | | Health and readiness checks |
| `config` | | Show current configuration |
| `install-claude-skill` | | Install Claude Code skill |

### Examples

```bash
# List workflows
n8n wf list

# Get workflow details
n8n wf get <workflow_id>

# Create workflow from JSON
n8n wf create workflow.json

# Run a webhook workflow
n8n wf run <workflow_id> -d '{"key": "value"}'

# Execute any workflow via API
n8n exec run <workflow_id>

# List executions
n8n exec list -w <workflow_id>

# Validate workflow
n8n wf validate --file workflow.json

# Edit workflow in external editor
n8n wf edit <workflow_id>

# Add a node
n8n wf nodes add <workflow_id> -t "n8n-nodes-base.httpRequest" -n "API Call"

# Add connection between nodes
n8n wf connections add <workflow_id> --from "Trigger" --to "API Call"
```

## Configuration

### Environment Variables

| Variable | Description |
|----------|-------------|
| `N8N_BASE_URL` | n8n instance URL |
| `N8N_API_KEY` | API key for authentication |
| `N8N_PROFILE` | Configuration profile to use |

### Config File

Create `~/.config/n8n-cli/config.toml`:

```toml
default_profile = "production"

[profiles.production]
base_url = "https://n8n.example.com"
api_key_env = "N8N_PROD_API_KEY"

[profiles.local]
base_url = "http://localhost:5678"
api_key = "local-dev-key"
```

Switch profiles with `-p`:

```bash
n8n -p local workflows list
```

## Output Formats

```bash
n8n wf list              # Table (default)
n8n wf list -o json      # JSON
n8n wf list -o json-pretty  # Pretty JSON
```

## Documentation

- [Getting Started](docs/getting-started.md)
- [Configuration](docs/configuration.md)
- [Command Reference](docs/commands/README.md)
- [Claude Skill Guide](docs/guides/claude-skill.md)
- [Scripting & Automation](docs/guides/scripting.md)

## Development

- [Architecture](docs/development/architecture.md)
- [Contributing](docs/development/contributing.md)

## License

Apache 2.0
