# Executions Commands

Monitor and manage workflow executions, including viewing history, getting details, retrying failures, and triggering new executions.

**Alias:** `exec`

## Commands

- [list](#list) - List executions
- [get](#get) - Get execution details
- [delete](#delete) - Delete an execution
- [retry](#retry) - Retry a failed execution
- [run](#run) - Execute a workflow via API

---

## list

List workflow executions with optional filtering.

```bash
n8n executions list [OPTIONS]
```

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--workflow-id <ID>` | `-w` | Filter by workflow ID |
| `--status <STATUS>` | `-s` | Filter by status |
| `--include-data` | | Include full execution data |
| `--limit <N>` | | Maximum results (default: 100) |
| `--cursor <CURSOR>` | | Pagination cursor |

### Status Values

| Status | Description |
|--------|-------------|
| `running` | Currently executing |
| `success` | Completed successfully |
| `error` | Failed with error |
| `waiting` | Waiting for external event |
| `canceled` | Manually cancelled |

### Examples

```bash
# List all recent executions
n8n exec list

# List executions for specific workflow
n8n exec list -w wf_abc123

# List only failed executions
n8n exec list --status error

# List successful executions for a workflow
n8n exec list -w wf_abc123 -s success

# Get last 10 executions as JSON
n8n exec list --limit 10 -o json

# Include execution data (large response)
n8n exec list -w wf_abc123 --include-data
```

### Output Columns

| Column | Description |
|--------|-------------|
| ID | Execution ID |
| Workflow | Workflow ID |
| Status | Execution status |
| Mode | How it was triggered (manual, webhook, trigger, etc.) |
| Started | Start timestamp |
| Finished | Whether execution has completed |

---

## get

Get detailed information about a specific execution.

```bash
n8n executions get <ID> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Execution ID |

### Options

| Option | Description |
|--------|-------------|
| `--include-data` | Include full execution data (input/output of each node) |

### Examples

```bash
# Get execution summary
n8n exec get exec_xyz789

# Get with full data
n8n exec get exec_xyz789 --include-data

# Get as JSON for analysis
n8n exec get exec_xyz789 --include-data -o json-pretty
```

### Execution Data

When using `--include-data`, the response includes:

- Input data for each node
- Output data for each node
- Error information (if failed)
- Timing information

This can be useful for debugging failed executions or analyzing workflow behavior.

---

## delete

Delete an execution from history.

```bash
n8n executions delete <ID>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Execution ID to delete |

### Examples

```bash
n8n exec delete exec_xyz789
```

### Use Cases

- Clean up test executions
- Remove executions with sensitive data
- Free up storage space

---

## retry

Retry a failed execution.

```bash
n8n executions retry <ID>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Execution ID to retry |

### Examples

```bash
# Retry a failed execution
n8n exec retry exec_xyz789

# Check the new execution
n8n exec list -w wf_abc123 --limit 1
```

### How Retry Works

1. Creates a new execution using the original input data
2. Runs the workflow from the beginning
3. Returns the new execution ID

**Note:** The original execution is preserved; retry creates a new execution.

---

## run

Execute a workflow directly via the n8n API.

```bash
n8n executions run <WORKFLOW_ID> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `WORKFLOW_ID` | ID of workflow to execute |

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--data <JSON>` | `-d` | Input data as JSON string |
| `--wait` | | Wait for execution to complete |

### Examples

```bash
# Execute workflow
n8n exec run wf_abc123

# Execute with input data
n8n exec run wf_abc123 -d '{"userId": 123, "action": "sync"}'

# Execute and wait for completion
n8n exec run wf_abc123 --wait

# Complex input data
n8n exec run wf_abc123 -d '{
  "items": [
    {"id": 1, "name": "Item 1"},
    {"id": 2, "name": "Item 2"}
  ],
  "options": {
    "notify": true,
    "format": "json"
  }
}'
```

### Comparison with `workflows run`

| Feature | `executions run` | `workflows run` |
|---------|-----------------|-----------------|
| Works with | Any workflow | Webhook workflows only |
| Method | n8n API `/execute` endpoint | Direct HTTP to webhook URL |
| Trigger type | Ignores trigger node | Uses webhook trigger |
| Use case | General execution | Testing webhooks |

### Input Data Format

The input data becomes available to the first node in the workflow. Structure depends on your workflow's expectations:

```json
{
  "key": "value",
  "nested": {
    "data": "here"
  },
  "array": [1, 2, 3]
}
```

---

## Scripting Examples

### Monitor Workflow Health

```bash
#!/bin/bash
# Check for recent failures

WORKFLOW_ID="wf_abc123"
FAILURES=$(n8n exec list -w "$WORKFLOW_ID" -s error --limit 10 -o json | jq length)

if [ "$FAILURES" -gt 5 ]; then
  echo "Warning: $FAILURES recent failures for workflow $WORKFLOW_ID"
  exit 1
fi
```

### Retry All Recent Failures

```bash
#!/bin/bash
# Retry all failed executions from today

n8n exec list --status error -o json | \
  jq -r '.[].id' | \
  while read exec_id; do
    echo "Retrying $exec_id..."
    n8n exec retry "$exec_id"
  done
```

### Wait for Execution Completion

```bash
#!/bin/bash
# Run workflow and poll for completion

EXEC_RESULT=$(n8n exec run wf_abc123 -d '{"test": true}' -o json)
EXEC_ID=$(echo "$EXEC_RESULT" | jq -r '.id')

echo "Started execution: $EXEC_ID"

while true; do
  STATUS=$(n8n exec get "$EXEC_ID" -o json | jq -r '.status')

  case "$STATUS" in
    "success")
      echo "Execution completed successfully"
      exit 0
      ;;
    "error")
      echo "Execution failed"
      exit 1
      ;;
    "running"|"waiting")
      echo "Status: $STATUS - waiting..."
      sleep 5
      ;;
  esac
done
```

### Export Execution Data

```bash
#!/bin/bash
# Export all execution data for a workflow

WORKFLOW_ID="wf_abc123"
OUTPUT_DIR="./executions"

mkdir -p "$OUTPUT_DIR"

n8n exec list -w "$WORKFLOW_ID" --all -o json | \
  jq -r '.[].id' | \
  while read exec_id; do
    n8n exec get "$exec_id" --include-data -o json-pretty \
      > "$OUTPUT_DIR/$exec_id.json"
    echo "Exported $exec_id"
  done
```

---

## See Also

- [Running Workflows Guide](../guides/running-workflows.md) - Detailed guide on executing workflows
- [Workflows Commands](./workflows.md) - Workflow management including `workflows run`
- [Scripting Guide](../guides/scripting.md) - More automation examples
