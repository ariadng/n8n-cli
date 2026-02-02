# n8n CLI Documentation

A powerful command-line interface for managing n8n workflows, executions, credentials, and tags.

## Table of Contents

### Getting Started

- [**Getting Started**](./getting-started.md) - Installation, prerequisites, and quick start guide
- [**Configuration**](./configuration.md) - Config files, environment variables, and profiles

### Command Reference

- [**Commands Overview**](./commands/README.md) - Command hierarchy and global flags
- [**Workflows**](./commands/workflows.md) - Manage workflows, nodes, and connections
- [**Executions**](./commands/executions.md) - Monitor and manage workflow executions
- [**Credentials**](./commands/credentials.md) - Manage n8n credentials
- [**Tags**](./commands/tags.md) - Organize workflows with tags
- [**Health**](./commands/health.md) - Health and readiness checks

### Guides

- [**Running Workflows**](./guides/running-workflows.md) - Execute and test workflows from the CLI
- [**Editing Workflows**](./guides/editing-workflows.md) - Edit workflows with external editors
- [**Scripting & Automation**](./guides/scripting.md) - Shell scripting, CI/CD, and automation patterns
- [**Claude Skill**](./guides/claude-skill.md) - AI-assisted workflow development with Claude Code

### Development

- [**Architecture**](./development/architecture.md) - High-level architecture overview
- [**Code Structure**](./development/code-structure.md) - Module organization and file purposes
- [**Adding Commands**](./development/adding-commands.md) - Step-by-step guide for adding new commands
- [**API Client**](./development/api-client.md) - HTTP client and endpoint implementation
- [**Models**](./development/models.md) - Data structures reference
- [**Error Handling**](./development/error-handling.md) - Error types and exit codes
- [**Contributing**](./development/contributing.md) - Development setup and contribution guidelines

## Quick Reference

### Common Commands

```bash
# List all workflows
n8n workflows list

# Get workflow details
n8n wf get <workflow_id>

# Run a webhook workflow
n8n wf run <workflow_id> -d '{"key": "value"}'

# List recent executions
n8n exec list -w <workflow_id>

# Check n8n health
n8n health check
```

### Configuration

```bash
# Set via environment variables
export N8N_BASE_URL="https://n8n.example.com"
export N8N_API_KEY="your-api-key"

# Or use CLI flags
n8n --url https://n8n.example.com --api-key your-api-key workflows list

# Or use a config file (~/.config/n8n-cli/config.toml)
```

### Output Formats

```bash
# Table output (default)
n8n workflows list

# JSON output
n8n workflows list -o json

# Pretty JSON output
n8n workflows list -o json-pretty
```

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

## Requirements

- A running n8n instance with API access enabled
- n8n API key (generated in n8n Settings > API)
- Rust 1.75+ (only if building from source)
