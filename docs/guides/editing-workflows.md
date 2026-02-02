# Editing Workflows

Edit n8n workflows using your favorite text editor directly from the command line.

## Overview

The `n8n workflows edit` command allows you to:

1. Download a workflow as JSON
2. Open it in your preferred editor
3. Make changes
4. Validate the changes
5. Upload back to n8n

This provides a powerful alternative to the n8n UI for users comfortable with JSON editing.

## Basic Usage

```bash
n8n workflows edit <workflow_id>
```

The workflow opens in your default editor. Save and close to upload changes.

## Editor Selection

### Default Behavior

The CLI checks these environment variables in order:

1. `$EDITOR`
2. `$VISUAL`
3. Falls back to `vi`

### Setting Your Editor

```bash
# In ~/.bashrc or ~/.zshrc
export EDITOR=vim
export EDITOR=nano
export EDITOR="code --wait"  # VS Code
export EDITOR="subl --wait"  # Sublime Text
```

### Per-Command Override

```bash
n8n wf edit wf_abc123 --editor vim
n8n wf edit wf_abc123 --editor "code --wait"
```

## Editor Configuration

### VS Code

```bash
# Must use --wait flag
export EDITOR="code --wait"

# Or specify directly
n8n wf edit wf_abc123 --editor "code --wait"
```

VS Code tips:
- Install JSON extension for better formatting
- Use `Ctrl+Shift+I` to format JSON
- Enable schema validation for n8n workflow files

### Vim/Neovim

```bash
export EDITOR=vim
# or
export EDITOR=nvim
```

Vim tips:
- `:set syntax=json` for highlighting
- `:%!jq .` to format JSON
- Use folding: `set foldmethod=syntax`

### Nano

```bash
export EDITOR=nano
```

### Sublime Text

```bash
export EDITOR="subl --wait"
```

### JetBrains IDEs

```bash
# IntelliJ IDEA
export EDITOR="idea --wait"

# WebStorm
export EDITOR="webstorm --wait"
```

## Workflow JSON Structure

Understanding the workflow format helps with editing:

```json
{
  "id": "wf_abc123",
  "name": "My Workflow",
  "active": false,
  "nodes": [
    {
      "id": "uuid-here",
      "name": "Start",
      "type": "n8n-nodes-base.manualTrigger",
      "position": [250, 300],
      "parameters": {},
      "typeVersion": 1
    },
    {
      "id": "another-uuid",
      "name": "HTTP Request",
      "type": "n8n-nodes-base.httpRequest",
      "position": [450, 300],
      "parameters": {
        "url": "https://api.example.com",
        "method": "GET"
      },
      "typeVersion": 4
    }
  ],
  "connections": {
    "Start": {
      "main": [
        [
          {
            "node": "HTTP Request",
            "type": "main",
            "index": 0
          }
        ]
      ]
    }
  },
  "settings": {
    "saveExecutionProgress": true,
    "saveDataErrorExecution": "all",
    "saveDataSuccessExecution": "all"
  }
}
```

### Key Sections

| Section | Description |
|---------|-------------|
| `name` | Workflow display name |
| `active` | Whether workflow is active |
| `nodes` | Array of node definitions |
| `connections` | Node connection map |
| `settings` | Workflow execution settings |

### Node Structure

| Field | Description |
|-------|-------------|
| `id` | Unique identifier (UUID) |
| `name` | Display name (must be unique) |
| `type` | n8n node type (e.g., `n8n-nodes-base.httpRequest`) |
| `position` | `[x, y]` coordinates on canvas |
| `parameters` | Node-specific configuration |
| `typeVersion` | Node version (don't change unless upgrading) |
| `disabled` | If `true`, node is skipped during execution |
| `credentials` | Credential references |

### Connection Structure

Connections are stored as a nested map:

```json
{
  "Source Node Name": {
    "main": [           // Output type (usually "main")
      [                 // Output index 0
        {
          "node": "Target Node Name",
          "type": "main",
          "index": 0    // Target input index
        }
      ],
      [                 // Output index 1 (for IF nodes, etc.)
        {
          "node": "Another Node",
          "type": "main",
          "index": 0
        }
      ]
    ]
  }
}
```

## Validation

By default, the CLI validates your changes before uploading.

### Validation Checks

| Check | Severity |
|-------|----------|
| Valid JSON | Error |
| No duplicate node IDs | Error |
| No duplicate node names | Error |
| No empty node names | Error |
| No empty workflow name | Error |
| Has trigger node | Warning |
| No orphan nodes | Warning |
| No self-loops | Warning |
| Valid connection references | Error |

### Skip Validation

```bash
n8n wf edit wf_abc123 --no-validate
```

Use cautiously - invalid workflows may fail to execute.

## Common Editing Tasks

### Add a Node

1. Create a unique ID (UUID format)
2. Add to `nodes` array
3. Add connections if needed

```json
{
  "id": "12345678-1234-1234-1234-123456789012",
  "name": "New Node",
  "type": "n8n-nodes-base.code",
  "position": [650, 300],
  "parameters": {
    "jsCode": "return items;"
  },
  "typeVersion": 2
}
```

### Remove a Node

1. Delete from `nodes` array
2. Remove all connections to/from the node

### Rename a Node

1. Update `name` in the node
2. Update references in `connections` (both source and target)

### Change Node Parameters

Update the `parameters` object:

```json
{
  "name": "HTTP Request",
  "parameters": {
    "url": "https://new-api.example.com",
    "method": "POST",
    "bodyParameters": {
      "parameters": [
        {"name": "key", "value": "value"}
      ]
    }
  }
}
```

### Reposition Nodes

Update the `position` array:

```json
{
  "name": "My Node",
  "position": [500, 400]
}
```

### Disable/Enable a Node

```json
{
  "name": "My Node",
  "disabled": true
}
```

### Add a Connection

Add to the connections map:

```json
{
  "connections": {
    "Source Node": {
      "main": [
        [
          {"node": "Target Node", "type": "main", "index": 0}
        ]
      ]
    }
  }
}
```

## Workflow Examples

### Minimal Workflow

```json
{
  "name": "Simple Workflow",
  "nodes": [
    {
      "id": "11111111-1111-1111-1111-111111111111",
      "name": "Start",
      "type": "n8n-nodes-base.manualTrigger",
      "position": [250, 300],
      "parameters": {}
    },
    {
      "id": "22222222-2222-2222-2222-222222222222",
      "name": "End",
      "type": "n8n-nodes-base.noOp",
      "position": [450, 300],
      "parameters": {}
    }
  ],
  "connections": {
    "Start": {
      "main": [[{"node": "End", "type": "main", "index": 0}]]
    }
  }
}
```

### IF Branch Workflow

```json
{
  "name": "Branching Workflow",
  "nodes": [
    {
      "id": "11111111-1111-1111-1111-111111111111",
      "name": "Start",
      "type": "n8n-nodes-base.manualTrigger",
      "position": [250, 300],
      "parameters": {}
    },
    {
      "id": "22222222-2222-2222-2222-222222222222",
      "name": "Check Condition",
      "type": "n8n-nodes-base.if",
      "position": [450, 300],
      "parameters": {
        "conditions": {
          "boolean": [{"value1": "={{$json.success}}", "value2": true}]
        }
      }
    },
    {
      "id": "33333333-3333-3333-3333-333333333333",
      "name": "Success",
      "type": "n8n-nodes-base.noOp",
      "position": [650, 200],
      "parameters": {}
    },
    {
      "id": "44444444-4444-4444-4444-444444444444",
      "name": "Failure",
      "type": "n8n-nodes-base.noOp",
      "position": [650, 400],
      "parameters": {}
    }
  ],
  "connections": {
    "Start": {
      "main": [[{"node": "Check Condition", "type": "main", "index": 0}]]
    },
    "Check Condition": {
      "main": [
        [{"node": "Success", "type": "main", "index": 0}],
        [{"node": "Failure", "type": "main", "index": 0}]
      ]
    }
  }
}
```

## Tips and Best Practices

### 1. Use Version Control

```bash
# Export before editing
n8n wf export wf_abc123 --file workflow.json --pretty

# Edit
n8n wf edit wf_abc123

# Compare changes
n8n wf diff wf_abc123 --file workflow.json
```

### 2. Generate UUIDs Properly

Use a proper UUID generator:

```bash
# macOS/Linux
uuidgen

# Python
python -c "import uuid; print(uuid.uuid4())"
```

### 3. Format JSON

Keep JSON readable:

```bash
# Using jq
cat workflow.json | jq . > formatted.json

# In vim
:%!jq .
```

### 4. Validate Before Editing

```bash
# Check current state
n8n wf validate wf_abc123 --warnings

# Edit
n8n wf edit wf_abc123
```

### 5. Use Quiet Mode for Scripting

```bash
n8n -q wf edit wf_abc123
```

## Troubleshooting

### "No changes detected"

The file wasn't modified or was restored to original content.

### "Validation failed"

Check the error message for specific issues. Common problems:
- Duplicate node names
- Invalid JSON syntax
- Missing required fields

### "Editor failed"

Ensure your editor command is correct:

```bash
# Test your editor
$EDITOR /tmp/test.json
```

For GUI editors, ensure the `--wait` flag is used.

### "Connection refers to non-existent node"

You renamed or deleted a node without updating connections.

---

## See Also

- [Workflows Commands](../commands/workflows.md) - Full command reference
- [Running Workflows](./running-workflows.md) - Test your edited workflows
- [Scripting Guide](./scripting.md) - Automation patterns
