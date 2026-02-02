# Configuration

The n8n CLI uses a layered configuration system that allows flexible setup for different environments and use cases.

## Configuration Hierarchy

Configuration values are resolved in this order (later sources override earlier):

1. **Built-in defaults** - Sensible defaults for all options
2. **Configuration file** - TOML file at `~/.config/n8n-cli/config.toml`
3. **Environment variables** - `N8N_*` prefixed variables
4. **CLI flags** - Command-line arguments

This means CLI flags always take precedence, followed by environment variables, then the config file, and finally defaults.

## Configuration Methods

### 1. Environment Variables

The simplest configuration method, ideal for CI/CD and containers:

| Variable | Description | Example |
|----------|-------------|---------|
| `N8N_BASE_URL` | n8n instance URL | `https://n8n.example.com` |
| `N8N_API_KEY` | API key for authentication | `n8n_api_xxx...` |
| `N8N_PROFILE` | Default profile to use | `production` |

Example:

```bash
export N8N_BASE_URL="https://n8n.example.com"
export N8N_API_KEY="n8n_api_abc123def456"

n8n workflows list
```

### 2. CLI Flags

Override any configuration for a single command:

| Flag | Short | Description |
|------|-------|-------------|
| `--url <URL>` | | n8n instance URL |
| `--api-key <KEY>` | | API key |
| `--profile <NAME>` | `-p` | Use named profile |
| `--output <FORMAT>` | `-o` | Output format: `table`, `json`, `json-pretty` |
| `--verbose` | `-v` | Enable verbose output |
| `--quiet` | `-q` | Suppress non-essential output |

Example:

```bash
n8n --url https://n8n.example.com --api-key abc123 workflows list
n8n -p staging -o json workflows list
```

### 3. Configuration File

Create a TOML file at `~/.config/n8n-cli/config.toml` for persistent configuration:

```toml
# Default profile to use when none specified
default_profile = "production"

# Default output format: "table", "json", or "json-pretty"
output_format = "table"

# HTTP timeout in seconds
timeout_secs = 30

# Named profiles for different n8n instances
[profiles.production]
base_url = "https://n8n.example.com"
api_key = "n8n_api_prod_key_here"

[profiles.staging]
base_url = "https://staging.n8n.example.com"
api_key = "n8n_api_staging_key_here"

[profiles.local]
base_url = "http://localhost:5678"
# Use environment variable for API key (more secure)
api_key_env = "N8N_LOCAL_API_KEY"
```

#### Configuration File Location

The config file location follows the XDG Base Directory specification:

| Platform | Path |
|----------|------|
| Linux | `~/.config/n8n-cli/config.toml` |
| macOS | `~/Library/Application Support/n8n-cli/config.toml` |
| Windows | `%APPDATA%\n8n-cli\config.toml` |

Create the directory if it doesn't exist:

```bash
mkdir -p ~/.config/n8n-cli
```

## Configuration File Reference

### Global Options

```toml
# The profile to use by default
# Can be overridden with -p/--profile flag or N8N_PROFILE env var
default_profile = "production"

# Default output format for commands
# Options: "table", "json", "json-pretty"
output_format = "table"

# HTTP request timeout in seconds
# Increase for slow connections or large responses
timeout_secs = 30
```

### Profile Options

Each profile can have these settings:

```toml
[profiles.myprofile]
# Required: The base URL of your n8n instance
# Do not include /api/v1 - it's added automatically
base_url = "https://n8n.example.com"

# Option 1: API key directly in config (less secure)
api_key = "n8n_api_your_key_here"

# Option 2: API key from environment variable (more secure)
# The CLI will read the value from this env var
api_key_env = "MY_N8N_API_KEY"
```

**Security Note:** Using `api_key_env` is recommended for production as it keeps secrets out of config files that might be accidentally committed to version control.

## Profiles

Profiles allow you to manage multiple n8n instances easily.

### Creating Profiles

Define profiles in your config file:

```toml
[profiles.production]
base_url = "https://n8n.company.com"
api_key_env = "N8N_PROD_API_KEY"

[profiles.development]
base_url = "http://localhost:5678"
api_key = "dev-key-here"

[profiles.client-a]
base_url = "https://n8n.client-a.com"
api_key_env = "N8N_CLIENT_A_KEY"
```

### Using Profiles

```bash
# Use default profile
n8n workflows list

# Use specific profile
n8n -p development workflows list
n8n --profile client-a workflows list

# Override with environment variable
N8N_PROFILE=staging n8n workflows list
```

### Profile Selection Priority

1. `--profile` / `-p` CLI flag
2. `N8N_PROFILE` environment variable
3. `default_profile` in config file
4. First profile found in config file

## Viewing Current Configuration

Display the resolved configuration:

```bash
n8n config
```

Output:
```
Current configuration:
  Base URL: https://n8n.example.com
  API Key: n8n_api_***...*** (configured)
  Profile: production
  Output: table
  Timeout: 30s
```

## Example Configurations

### Simple Single-Instance Setup

For a single n8n instance, environment variables are sufficient:

```bash
# ~/.bashrc or ~/.zshrc
export N8N_BASE_URL="https://n8n.example.com"
export N8N_API_KEY="your-api-key"
```

### Multi-Environment Setup

For teams with multiple environments:

```toml
# ~/.config/n8n-cli/config.toml
default_profile = "development"
output_format = "table"
timeout_secs = 30

[profiles.development]
base_url = "http://localhost:5678"
api_key_env = "N8N_DEV_KEY"

[profiles.staging]
base_url = "https://staging-n8n.example.com"
api_key_env = "N8N_STAGING_KEY"

[profiles.production]
base_url = "https://n8n.example.com"
api_key_env = "N8N_PROD_KEY"
```

### CI/CD Setup

For automated pipelines, use only environment variables:

```yaml
# GitHub Actions example
env:
  N8N_BASE_URL: ${{ secrets.N8N_URL }}
  N8N_API_KEY: ${{ secrets.N8N_API_KEY }}

steps:
  - name: Deploy workflow
    run: |
      n8n workflows update wf_123 workflow.json
      n8n workflows activate wf_123
```

### Docker/Container Setup

Pass configuration via environment:

```bash
docker run -e N8N_BASE_URL=https://n8n.example.com \
           -e N8N_API_KEY=your-key \
           n8n-cli workflows list
```

## Defaults Reference

When no configuration is provided:

| Setting | Default Value |
|---------|---------------|
| `base_url` | `http://localhost:5678` |
| `api_key` | None (required) |
| `output_format` | `table` |
| `timeout_secs` | `30` |

## Security Best Practices

1. **Never commit API keys** - Use `api_key_env` or environment variables
2. **Use environment variables in CI/CD** - Leverage your platform's secrets management
3. **Restrict config file permissions** - `chmod 600 ~/.config/n8n-cli/config.toml`
4. **Rotate API keys regularly** - Generate new keys periodically in n8n settings
5. **Use read-only keys when possible** - If n8n supports scoped keys, use minimal permissions
