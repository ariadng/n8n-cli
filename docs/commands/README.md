# Commands Overview

The n8n CLI organizes commands into logical groups for managing different aspects of your n8n instance.

## Command Hierarchy

```
n8n
├── workflows (wf)          # Manage workflows
│   ├── list                # List all workflows
│   ├── get                 # Get workflow details
│   ├── create              # Create from JSON file
│   ├── update              # Update existing workflow
│   ├── delete              # Delete a workflow
│   ├── activate            # Activate a workflow
│   ├── deactivate          # Deactivate a workflow
│   ├── edit                # Edit in external editor
│   ├── export              # Export to file
│   ├── clone               # Duplicate a workflow
│   ├── validate            # Validate workflow structure
│   ├── diff                # Compare workflows
│   ├── run                 # Trigger webhook workflow
│   ├── nodes               # Manage nodes
│   │   ├── list            # List nodes in workflow
│   │   ├── get             # Get node details
│   │   ├── add             # Add new node
│   │   ├── remove          # Remove node
│   │   ├── update          # Update node config
│   │   └── move            # Reposition node
│   └── connections         # Manage connections
│       ├── list            # List connections
│       ├── add             # Add connection
│       └── remove          # Remove connection
│
├── executions (exec)       # Manage executions
│   ├── list                # List executions
│   ├── get                 # Get execution details
│   ├── delete              # Delete execution
│   ├── retry               # Retry failed execution
│   └── run                 # Execute workflow via API
│
├── credentials (cred)      # Manage credentials
│   ├── list                # List credentials
│   ├── schema              # Get credential type schema
│   ├── create              # Create credential
│   ├── update              # Update credential
│   └── delete              # Delete credential
│
├── tags                    # Manage tags
│   ├── list                # List tags
│   ├── create              # Create tag
│   ├── update              # Update tag
│   ├── delete              # Delete tag
│   └── assign              # Assign tags to workflow
│
├── health                  # Health checks
│   ├── check               # Basic health check
│   └── ready               # Readiness check
│
└── config                  # Show configuration
```

## Global Flags

These flags can be used with any command:

| Flag | Short | Env Variable | Description |
|------|-------|--------------|-------------|
| `--profile <NAME>` | `-p` | `N8N_PROFILE` | Use named configuration profile |
| `--url <URL>` | | `N8N_BASE_URL` | n8n instance URL |
| `--api-key <KEY>` | | `N8N_API_KEY` | API key for authentication |
| `--output <FORMAT>` | `-o` | | Output format: `table`, `json`, `json-pretty` |
| `--verbose` | `-v` | | Enable verbose output |
| `--quiet` | `-q` | | Suppress non-essential output |
| `--help` | `-h` | | Show help information |
| `--version` | `-V` | | Show version information |

### Examples

```bash
# Use a specific profile
n8n -p production workflows list

# Override URL for this command
n8n --url https://other-n8n.com workflows list

# Output as JSON
n8n -o json workflows list

# Combine flags
n8n -p staging -o json-pretty workflows get wf_123

# Quiet mode (only output data, no status messages)
n8n -q workflows list
```

## Output Formats

### Table (Default)

Human-readable tabular format:

```bash
n8n workflows list
```

```
┌────────────────────────┬─────────────────────┬────────┬─────────────────────┐
│ ID                     │ Name                │ Active │ Updated             │
├────────────────────────┼─────────────────────┼────────┼─────────────────────┤
│ wf_abc123              │ My Workflow         │ true   │ 2024-01-15 10:30:00 │
└────────────────────────┴─────────────────────┴────────┴─────────────────────┘
```

### JSON

Compact JSON for scripting and piping:

```bash
n8n -o json workflows list
```

```json
[{"id":"wf_abc123","name":"My Workflow","active":true,"updatedAt":"2024-01-15T10:30:00Z"}]
```

### JSON Pretty

Formatted JSON for readability:

```bash
n8n -o json-pretty workflows list
```

```json
[
  {
    "id": "wf_abc123",
    "name": "My Workflow",
    "active": true,
    "updatedAt": "2024-01-15T10:30:00Z"
  }
]
```

## Command Aliases

For convenience, common commands have short aliases:

| Command | Alias |
|---------|-------|
| `workflows` | `wf` |
| `executions` | `exec` |
| `credentials` | `cred` |

```bash
n8n wf list          # Same as: n8n workflows list
n8n exec list        # Same as: n8n executions list
n8n cred list        # Same as: n8n credentials list
```

## Getting Help

### General Help

```bash
n8n --help
```

### Command Group Help

```bash
n8n workflows --help
n8n executions --help
```

### Subcommand Help

```bash
n8n workflows list --help
n8n workflows nodes add --help
```

## Exit Codes

The CLI uses standard Unix exit codes:

| Code | Meaning | Example Cause |
|------|---------|---------------|
| 0 | Success | Command completed successfully |
| 1 | General error | API error, unexpected failure |
| 65 | Data error | Invalid JSON, parse failure |
| 69 | Unavailable | Resource not found (404) |
| 74 | I/O error | File read/write failure |
| 77 | Permission denied | Authentication failed (401/403) |
| 78 | Configuration error | Invalid config, missing required values |
| 130 | Cancelled | User interrupted (Ctrl+C) |

See [Error Handling](../development/error-handling.md) for complete details.

## Common Patterns

### Filtering and Pagination

Many list commands support filtering:

```bash
# Filter workflows by active status
n8n wf list --active true

# Filter by tags
n8n wf list --tags production,critical

# Filter by name (partial match)
n8n wf list --name "data sync"

# Limit results
n8n wf list --limit 10

# Fetch all pages
n8n wf list --all
```

### Working with IDs

Most commands accept resource IDs:

```bash
n8n wf get wf_abc123
n8n exec get exec_xyz789
n8n wf nodes list wf_abc123
```

### File Input/Output

Many commands support file operations:

```bash
# Read from file
n8n wf create workflow.json

# Read from stdin
cat workflow.json | n8n wf create -

# Write to file
n8n wf export wf_123 --file output.json

# Write to stdout (default)
n8n wf export wf_123
```

### Confirmation Prompts

Destructive operations ask for confirmation:

```bash
n8n wf delete wf_123
# Are you sure you want to delete workflow 'My Workflow'? [y/N]

# Skip confirmation with --force
n8n wf delete wf_123 --force
```

## Next Steps

- [Workflows](./workflows.md) - Complete workflow command reference
- [Executions](./executions.md) - Execution management commands
- [Credentials](./credentials.md) - Credential management
- [Tags](./tags.md) - Tag management
- [Health](./health.md) - Health check commands
