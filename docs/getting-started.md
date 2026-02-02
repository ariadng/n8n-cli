# Getting Started

This guide walks you through installing the n8n CLI and making your first API calls.

## Prerequisites

### 1. Rust Toolchain

The n8n CLI is built with Rust. Install Rust using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:

```bash
rustc --version  # Should be 1.75 or later
cargo --version
```

### 2. n8n Instance

You need a running n8n instance with API access enabled. The CLI supports:

- Self-hosted n8n instances
- n8n Cloud

### 3. API Key

Generate an API key in your n8n instance:

1. Go to **Settings** > **API**
2. Click **Create API Key**
3. Copy the generated key (you won't see it again)

## Installation

### From Source

Clone and build:

```bash
git clone https://github.com/your-org/n8n-cli.git
cd n8n-cli
cargo build --release
```

The binary will be at `target/release/n8n`. Add it to your PATH:

```bash
# Option 1: Copy to a directory in PATH
sudo cp target/release/n8n /usr/local/bin/

# Option 2: Add target/release to PATH
export PATH="$PATH:$(pwd)/target/release"
```

### Using Cargo Install

```bash
cargo install --path .
```

This installs the `n8n` binary to `~/.cargo/bin/`.

## Configuration

### Quick Setup with Environment Variables

The fastest way to get started:

```bash
export N8N_BASE_URL="https://your-n8n-instance.com"
export N8N_API_KEY="your-api-key-here"
```

Add these to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.) to persist them.

### Verify Connection

Test your setup:

```bash
# Check n8n is reachable
n8n health check

# List workflows
n8n workflows list
```

If successful, you'll see your workflows in a table format.

### Configuration File (Optional)

For more advanced setups, create a config file at `~/.config/n8n-cli/config.toml`:

```toml
default_profile = "production"
output_format = "table"
timeout_secs = 30

[profiles.production]
base_url = "https://n8n.example.com"
api_key = "your-production-api-key"

[profiles.local]
base_url = "http://localhost:5678"
api_key = "your-local-api-key"
```

Switch between profiles:

```bash
n8n -p local workflows list      # Use local profile
n8n -p production workflows list # Use production profile
```

See [Configuration](./configuration.md) for full details.

## Your First Commands

### List Workflows

```bash
n8n workflows list
```

Output:
```
┌────────────────────────┬─────────────────────┬────────┬─────────────────────┐
│ ID                     │ Name                │ Active │ Updated             │
├────────────────────────┼─────────────────────┼────────┼─────────────────────┤
│ wf_abc123              │ My First Workflow   │ true   │ 2024-01-15 10:30:00 │
│ wf_def456              │ Data Sync           │ false  │ 2024-01-14 15:45:00 │
└────────────────────────┴─────────────────────┴────────┴─────────────────────┘
```

### Get Workflow Details

```bash
n8n workflows get wf_abc123
```

### Export Workflow to JSON

```bash
# To stdout
n8n workflows export wf_abc123

# To file
n8n workflows export wf_abc123 --file workflow.json --pretty
```

### Run a Workflow

For webhook-triggered workflows:

```bash
n8n workflows run wf_abc123 -d '{"message": "Hello from CLI"}'
```

For any workflow via API:

```bash
n8n executions run wf_abc123 -d '{"input": "data"}'
```

### Check Execution Status

```bash
# List recent executions
n8n executions list -w wf_abc123

# Get execution details
n8n executions get exec_789
```

## Command Aliases

The CLI provides short aliases for common commands:

| Full Command | Alias |
|-------------|-------|
| `workflows` | `wf` |
| `executions` | `exec` |
| `credentials` | `cred` |

Example:
```bash
n8n wf list        # Same as: n8n workflows list
n8n exec list      # Same as: n8n executions list
n8n cred list      # Same as: n8n credentials list
```

## Getting Help

### General Help

```bash
n8n --help
```

### Command-Specific Help

```bash
n8n workflows --help
n8n workflows list --help
n8n workflows nodes --help
```

## Next Steps

- [Configure multiple profiles](./configuration.md) for different environments
- [Learn all workflow commands](./commands/workflows.md)
- [Automate with shell scripts](./guides/scripting.md)
- [Edit workflows with your favorite editor](./guides/editing-workflows.md)

## Troubleshooting

### Connection Refused

```
Error: Connection failed: Connection refused
```

- Verify n8n is running and accessible at the configured URL
- Check firewall rules
- Try `curl https://your-n8n-url/healthz`

### Authentication Failed

```
Error: Permission denied (401)
```

- Verify your API key is correct
- Ensure API access is enabled in n8n settings
- Check the API key hasn't expired

### Certificate Errors

```
Error: Certificate verification failed
```

For self-signed certificates in development:
- Use proper certificates in production
- Consider using a reverse proxy with valid certificates
