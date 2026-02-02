# Workflows Commands

Manage n8n workflows including creation, updates, activation, and detailed node/connection manipulation.

**Alias:** `wf`

## Commands

- [list](#list) - List all workflows
- [get](#get) - Get workflow details
- [create](#create) - Create workflow from JSON
- [update](#update) - Update existing workflow
- [delete](#delete) - Delete a workflow
- [activate](#activate) - Activate a workflow
- [deactivate](#deactivate) - Deactivate a workflow
- [edit](#edit) - Edit in external editor
- [export](#export) - Export to file
- [clone](#clone) - Duplicate a workflow
- [validate](#validate) - Validate workflow structure
- [diff](#diff) - Compare workflows
- [run](#run) - Trigger webhook workflow
- [nodes](#nodes-subcommands) - Manage workflow nodes
- [connections](#connections-subcommands) - Manage connections

---

## list

List all workflows with optional filtering.

```bash
n8n workflows list [OPTIONS]
```

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--active <BOOL>` | `-a` | Filter by active status (`true`/`false`) |
| `--tags <TAGS>` | `-t` | Filter by tags (comma-separated) |
| `--name <NAME>` | `-n` | Filter by name (partial match) |
| `--limit <N>` | | Maximum results (default: 100) |
| `--cursor <CURSOR>` | | Pagination cursor |
| `--all` | | Fetch all pages automatically |

### Examples

```bash
# List all workflows
n8n wf list

# List only active workflows
n8n wf list --active true

# Filter by tags
n8n wf list -t production,critical

# Search by name
n8n wf list -n "data sync"

# Get first 10 results as JSON
n8n wf list --limit 10 -o json

# Fetch all workflows (auto-paginate)
n8n wf list --all
```

---

## get

Get detailed information about a single workflow.

```bash
n8n workflows get <ID>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Workflow ID |

### Examples

```bash
# Get workflow details
n8n wf get wf_abc123

# Get as JSON for scripting
n8n wf get wf_abc123 -o json
```

---

## create

Create a new workflow from a JSON file.

```bash
n8n workflows create <FILE> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `FILE` | Path to workflow JSON file (use `-` for stdin) |

### Options

| Option | Description |
|--------|-------------|
| `--activate` | Activate workflow immediately after creation |

### Examples

```bash
# Create from file
n8n wf create workflow.json

# Create and activate
n8n wf create workflow.json --activate

# Create from stdin
cat workflow.json | n8n wf create -

# Create from heredoc
n8n wf create - <<EOF
{
  "name": "My New Workflow",
  "nodes": [],
  "connections": {}
}
EOF
```

### Workflow JSON Structure

```json
{
  "name": "Workflow Name",
  "nodes": [
    {
      "id": "uuid-here",
      "name": "Start",
      "type": "n8n-nodes-base.manualTrigger",
      "position": [250, 300],
      "parameters": {}
    }
  ],
  "connections": {},
  "settings": {
    "saveExecutionProgress": true
  }
}
```

---

## update

Update an existing workflow.

```bash
n8n workflows update <ID> <FILE>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Workflow ID to update |
| `FILE` | Path to workflow JSON file (use `-` for stdin) |

### Examples

```bash
# Update from file
n8n wf update wf_abc123 updated-workflow.json

# Update from stdin
cat modified.json | n8n wf update wf_abc123 -
```

---

## delete

Delete a workflow.

```bash
n8n workflows delete <ID> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Workflow ID to delete |

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--force` | `-f` | Skip confirmation prompt |

### Examples

```bash
# Delete with confirmation
n8n wf delete wf_abc123

# Delete without confirmation
n8n wf delete wf_abc123 --force
```

---

## activate

Activate a workflow (enable triggers).

```bash
n8n workflows activate <ID>
```

### Examples

```bash
n8n wf activate wf_abc123
```

---

## deactivate

Deactivate a workflow (disable triggers).

```bash
n8n workflows deactivate <ID>
```

### Examples

```bash
n8n wf deactivate wf_abc123
```

---

## edit

Open a workflow in an external editor, then upload changes.

```bash
n8n workflows edit <ID> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Workflow ID to edit |

### Options

| Option | Description |
|--------|-------------|
| `--editor <EDITOR>` | Editor command (defaults to `$EDITOR` or `$VISUAL`) |
| `--no-validate` | Skip validation before uploading |

### Examples

```bash
# Edit with default editor
n8n wf edit wf_abc123

# Edit with specific editor
n8n wf edit wf_abc123 --editor code

# Edit with vim
n8n wf edit wf_abc123 --editor vim

# Skip validation
n8n wf edit wf_abc123 --no-validate
```

### How It Works

1. Downloads workflow JSON to a temporary file
2. Opens the file in your editor
3. Waits for editor to close
4. Validates the modified JSON (unless `--no-validate`)
5. Uploads changes to n8n

See [Editing Workflows Guide](../guides/editing-workflows.md) for detailed usage.

---

## export

Export a workflow to a file or stdout.

```bash
n8n workflows export <ID> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Workflow ID to export |

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--file <PATH>` | `-f` | Output file path |
| `--pretty` | | Pretty-print JSON |

### Examples

```bash
# Export to stdout
n8n wf export wf_abc123

# Export to file
n8n wf export wf_abc123 --file workflow.json

# Export with pretty formatting
n8n wf export wf_abc123 --file workflow.json --pretty

# Pipe to other commands
n8n wf export wf_abc123 | jq '.nodes | length'
```

---

## clone

Create a copy of an existing workflow.

```bash
n8n workflows clone <ID> --name <NAME> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Source workflow ID |

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--name <NAME>` | `-n` | Name for the cloned workflow (required) |
| `--activate` | | Activate the cloned workflow |

### Examples

```bash
# Clone with new name
n8n wf clone wf_abc123 --name "My Workflow (Copy)"

# Clone and activate
n8n wf clone wf_abc123 -n "Production Copy" --activate
```

---

## validate

Validate workflow structure without uploading.

```bash
n8n workflows validate [ID] [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Workflow ID to validate (optional if using `--file`) |

### Options

| Option | Description |
|--------|-------------|
| `--file <PATH>` | Validate local file instead |
| `--warnings` | Show warnings (not just errors) |

### Examples

```bash
# Validate workflow on server
n8n wf validate wf_abc123

# Validate local file
n8n wf validate --file workflow.json

# Show all issues including warnings
n8n wf validate --file workflow.json --warnings
```

### Validation Checks

| Check | Severity | Description |
|-------|----------|-------------|
| Empty workflow | Warning | Workflow has no nodes |
| Duplicate node IDs | Error | Multiple nodes share same ID |
| Duplicate node names | Error | Multiple nodes share same name |
| No trigger node | Warning | Workflow has no trigger to start execution |
| Invalid connection | Error | Connection references non-existent node |
| Orphan node | Warning | Node not connected to workflow |
| Self-loop | Warning | Node connects to itself |
| Empty node name | Error | Node has empty name |
| Empty workflow name | Error | Workflow has empty name |

---

## diff

Compare two workflows or a workflow with a local file.

```bash
n8n workflows diff <ID> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | First workflow ID |

### Options

| Option | Description |
|--------|-------------|
| `--with <ID>` | Second workflow ID to compare |
| `--file <PATH>` | Local file to compare against |
| `--full` | Show full diff (not just summary) |

### Examples

```bash
# Compare two workflows
n8n wf diff wf_abc123 --with wf_def456

# Compare workflow with local file
n8n wf diff wf_abc123 --file local-version.json

# Show detailed diff
n8n wf diff wf_abc123 --with wf_def456 --full
```

### Output

Summary mode shows:
- Name changes
- Active status changes
- Nodes added/removed/modified
- Connections added/removed

Full mode additionally shows:
- Parameter-level differences
- Unified diff format for changed values

---

## run

Trigger a webhook-based workflow.

```bash
n8n workflows run <ID> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Workflow ID to run |

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--data <JSON>` | `-d` | Input data as JSON string |
| `--data-file <PATH>` | | Input data from file |
| `--method <METHOD>` | `-m` | HTTP method (default: POST) |
| `--no-wait` | | Don't wait for completion |

### Examples

```bash
# Run with inline data
n8n wf run wf_abc123 -d '{"message": "Hello"}'

# Run with data from file
n8n wf run wf_abc123 --data-file input.json

# Run with GET method
n8n wf run wf_abc123 -m GET

# Fire and forget
n8n wf run wf_abc123 -d '{"event": "trigger"}' --no-wait
```

### Requirements

The workflow must have a **Webhook** or **Form Trigger** node. For other trigger types, use `n8n executions run` instead.

See [Running Workflows Guide](../guides/running-workflows.md) for more details.

---

## Nodes Subcommands

Manage individual nodes within a workflow.

### nodes list

List all nodes in a workflow.

```bash
n8n workflows nodes list <WORKFLOW_ID>
```

#### Examples

```bash
n8n wf nodes list wf_abc123
n8n wf nodes list wf_abc123 -o json
```

### nodes get

Get details of a specific node.

```bash
n8n workflows nodes get <WORKFLOW_ID> <NODE_ID>
```

#### Arguments

| Argument | Description |
|----------|-------------|
| `WORKFLOW_ID` | Workflow ID |
| `NODE_ID` | Node ID or node name |

#### Examples

```bash
n8n wf nodes get wf_abc123 node_xyz
n8n wf nodes get wf_abc123 "HTTP Request"
```

### nodes add

Add a new node to a workflow.

```bash
n8n workflows nodes add <WORKFLOW_ID> [OPTIONS]
```

#### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--type <TYPE>` | `-t` | n8n node type (required) |
| `--name <NAME>` | `-n` | Node display name (required) |
| `--position <X,Y>` | | Position as "x,y" (e.g., "200,300") |
| `--config <JSON>` | `-c` | Node parameters as JSON |
| `--config-file <PATH>` | | Node parameters from file |
| `--disabled` | | Create node as disabled |

#### Examples

```bash
# Add HTTP Request node
n8n wf nodes add wf_abc123 \
  -t "n8n-nodes-base.httpRequest" \
  -n "Fetch Data" \
  --position "400,300"

# Add with configuration
n8n wf nodes add wf_abc123 \
  -t "n8n-nodes-base.httpRequest" \
  -n "API Call" \
  -c '{"url": "https://api.example.com", "method": "GET"}'

# Add disabled node
n8n wf nodes add wf_abc123 \
  -t "n8n-nodes-base.code" \
  -n "Debug" \
  --disabled
```

#### Common Node Types

| Type | Description |
|------|-------------|
| `n8n-nodes-base.manualTrigger` | Manual trigger |
| `n8n-nodes-base.webhook` | Webhook trigger |
| `n8n-nodes-base.scheduleTrigger` | Scheduled trigger |
| `n8n-nodes-base.httpRequest` | HTTP Request |
| `n8n-nodes-base.code` | Code (JavaScript) |
| `n8n-nodes-base.if` | IF condition |
| `n8n-nodes-base.merge` | Merge data |
| `n8n-nodes-base.set` | Set node |

### nodes remove

Remove a node from a workflow.

```bash
n8n workflows nodes remove <WORKFLOW_ID> <NODE_ID> [OPTIONS]
```

#### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--force` | `-f` | Skip confirmation |

#### Examples

```bash
n8n wf nodes remove wf_abc123 node_xyz
n8n wf nodes remove wf_abc123 "Old Node" --force
```

**Note:** Removing a node also removes all its connections.

### nodes update

Update a node's configuration.

```bash
n8n workflows nodes update <WORKFLOW_ID> <NODE_ID> [OPTIONS]
```

#### Options

| Option | Description |
|--------|-------------|
| `--name <NAME>` | New node name |
| `--position <X,Y>` | New position |
| `--config <JSON>` | Parameters to merge (or replace) |
| `--replace` | Replace entire config instead of merging |
| `--disabled <BOOL>` | Enable/disable node |

#### Examples

```bash
# Rename node
n8n wf nodes update wf_abc123 node_xyz --name "New Name"

# Update parameters (merge)
n8n wf nodes update wf_abc123 node_xyz \
  -c '{"url": "https://new-api.example.com"}'

# Replace all parameters
n8n wf nodes update wf_abc123 node_xyz \
  -c '{"url": "https://api.example.com", "method": "POST"}' \
  --replace

# Disable node
n8n wf nodes update wf_abc123 node_xyz --disabled true

# Move node
n8n wf nodes update wf_abc123 node_xyz --position "500,400"
```

### nodes move

Move a node to a new position.

```bash
n8n workflows nodes move <WORKFLOW_ID> <NODE_ID> <POSITION>
```

#### Arguments

| Argument | Description |
|----------|-------------|
| `WORKFLOW_ID` | Workflow ID |
| `NODE_ID` | Node ID or name |
| `POSITION` | New position as "x,y" |

#### Examples

```bash
n8n wf nodes move wf_abc123 node_xyz "600,400"
n8n wf nodes move wf_abc123 "HTTP Request" "300,200"
```

---

## Connections Subcommands

Manage connections between nodes.

### connections list

List all connections in a workflow.

```bash
n8n workflows connections list <WORKFLOW_ID> [OPTIONS]
```

#### Options

| Option | Description |
|--------|-------------|
| `--from <NODE>` | Filter by source node |
| `--to <NODE>` | Filter by target node |

#### Examples

```bash
# List all connections
n8n wf connections list wf_abc123

# Filter by source
n8n wf connections list wf_abc123 --from "HTTP Request"

# Filter by target
n8n wf connections list wf_abc123 --to "Code"
```

### connections add

Add a connection between two nodes.

```bash
n8n workflows connections add <WORKFLOW_ID> [OPTIONS]
```

#### Options

| Option | Description |
|--------|-------------|
| `--from <NODE>` | Source node ID or name (required) |
| `--to <NODE>` | Target node ID or name (required) |
| `--output-index <N>` | Source output index (default: 0) |
| `--input-index <N>` | Target input index (default: 0) |
| `--type <TYPE>` | Connection type (default: "main") |

#### Examples

```bash
# Simple connection
n8n wf connections add wf_abc123 \
  --from "Trigger" \
  --to "HTTP Request"

# Connect specific outputs/inputs
n8n wf connections add wf_abc123 \
  --from "IF" \
  --to "Success Handler" \
  --output-index 0

n8n wf connections add wf_abc123 \
  --from "IF" \
  --to "Error Handler" \
  --output-index 1
```

### connections remove

Remove a connection.

```bash
n8n workflows connections remove <WORKFLOW_ID> [OPTIONS]
```

#### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--from <NODE>` | | Source node (required) |
| `--to <NODE>` | | Target node (required) |
| `--force` | `-f` | Skip confirmation |

#### Examples

```bash
n8n wf connections remove wf_abc123 \
  --from "Old Node" \
  --to "Next Node"

n8n wf connections remove wf_abc123 \
  --from node_a \
  --to node_b \
  --force
```
