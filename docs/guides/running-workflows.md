# Running Workflows

This guide covers different methods for executing n8n workflows from the CLI.

## Overview

The n8n CLI provides two commands for running workflows:

| Command | Use Case | Works With |
|---------|----------|------------|
| `n8n workflows run` | Test webhook endpoints | Webhook/Form trigger workflows |
| `n8n executions run` | Execute any workflow | All workflows |

## Method 1: Webhook Trigger (`workflows run`)

Use this command to trigger workflows that have a **Webhook** or **Form Trigger** node.

### Basic Usage

```bash
n8n workflows run <workflow_id>
```

### With Input Data

```bash
# Inline JSON
n8n wf run wf_abc123 -d '{"name": "John", "action": "signup"}'

# From file
n8n wf run wf_abc123 --data-file payload.json
```

### HTTP Methods

By default, the CLI sends a POST request. Specify a different method:

```bash
n8n wf run wf_abc123 -m GET
n8n wf run wf_abc123 -m PUT -d '{"status": "updated"}'
```

### Fire and Forget

Don't wait for the workflow to complete:

```bash
n8n wf run wf_abc123 -d '{"event": "trigger"}' --no-wait
```

### How It Works

1. CLI fetches the workflow to find the webhook node
2. Extracts the webhook path from node parameters
3. Makes an HTTP request to the webhook URL
4. Returns the response (unless `--no-wait`)

### Requirements

The workflow must:
- Have a Webhook or Form Trigger node
- Be **active** (triggers only work when workflow is active)

### Common Errors

**"Workflow has no webhook trigger"**
```
Error: Workflow has no webhook trigger. Only webhook workflows can be run via CLI.
Hint: Add a Webhook or Form Trigger node, or run manually in n8n UI.
```

Solution: Use `n8n executions run` instead, or add a webhook trigger to the workflow.

---

## Method 2: API Execution (`executions run`)

Use this command to execute any workflow directly via the n8n API.

### Basic Usage

```bash
n8n executions run <workflow_id>
```

### With Input Data

```bash
n8n exec run wf_abc123 -d '{"input": "data"}'
```

### Wait for Completion

```bash
n8n exec run wf_abc123 --wait
```

### How It Works

1. CLI calls the n8n API's execute endpoint
2. n8n creates a new execution
3. Returns execution details (ID, status, etc.)

### When to Use

- Workflows without webhook triggers (schedule, manual, etc.)
- Testing workflows programmatically
- CI/CD pipelines
- Scheduled jobs via cron

---

## Passing Input Data

Both commands accept JSON input data that becomes available to the workflow.

### Simple Data

```bash
n8n exec run wf_abc123 -d '{"userId": 123}'
```

### Complex Structures

```bash
n8n exec run wf_abc123 -d '{
  "user": {
    "id": 123,
    "name": "John Doe",
    "email": "john@example.com"
  },
  "items": [
    {"sku": "ABC", "qty": 2},
    {"sku": "XYZ", "qty": 1}
  ],
  "options": {
    "notify": true,
    "priority": "high"
  }
}'
```

### From File

```bash
# Create payload file
cat > payload.json << 'EOF'
{
  "action": "process",
  "data": {
    "records": [1, 2, 3, 4, 5]
  }
}
EOF

# Use with workflows run
n8n wf run wf_abc123 --data-file payload.json

# Or with executions run (inline)
n8n exec run wf_abc123 -d "$(cat payload.json)"
```

### From Environment Variables

```bash
export USER_DATA='{"userId": 123}'
n8n exec run wf_abc123 -d "$USER_DATA"
```

### Generated Data

```bash
# Current timestamp
n8n exec run wf_abc123 -d "{\"timestamp\": \"$(date -Iseconds)\"}"

# Random ID
n8n exec run wf_abc123 -d "{\"requestId\": \"$(uuidgen)\"}"
```

---

## Monitoring Executions

After triggering a workflow, monitor its progress:

### Get Execution Status

```bash
# Get execution details
n8n exec get <execution_id>

# Include full data (node inputs/outputs)
n8n exec get <execution_id> --include-data
```

### List Recent Executions

```bash
# All executions for a workflow
n8n exec list -w wf_abc123

# Only failed executions
n8n exec list -w wf_abc123 -s error

# Only successful
n8n exec list -w wf_abc123 -s success
```

### Poll for Completion

```bash
#!/bin/bash

EXEC_ID=$(n8n exec run wf_abc123 -o json | jq -r '.id')
echo "Started: $EXEC_ID"

while true; do
  STATUS=$(n8n exec get "$EXEC_ID" -o json | jq -r '.status')

  case "$STATUS" in
    success) echo "Completed!"; exit 0 ;;
    error)   echo "Failed!"; exit 1 ;;
    *)       echo "Status: $STATUS"; sleep 2 ;;
  esac
done
```

---

## Practical Examples

### Test a Webhook Workflow

```bash
# Activate the workflow first
n8n wf activate wf_abc123

# Trigger with test data
n8n wf run wf_abc123 -d '{"test": true, "source": "cli"}'

# Check execution
n8n exec list -w wf_abc123 --limit 1
```

### Batch Processing

```bash
#!/bin/bash
# Process multiple items

WORKFLOW="wf_abc123"

for item in item1 item2 item3; do
  echo "Processing: $item"
  n8n exec run "$WORKFLOW" -d "{\"item\": \"$item\"}"
  sleep 1  # Rate limiting
done
```

### CI/CD Deployment Test

```bash
#!/bin/bash
# Test workflow after deployment

WORKFLOW="wf_abc123"

# Deploy
n8n wf update "$WORKFLOW" workflow.json
n8n wf activate "$WORKFLOW"

# Test
RESULT=$(n8n exec run "$WORKFLOW" -d '{"test": true}' -o json)
EXEC_ID=$(echo "$RESULT" | jq -r '.id')

# Wait and check
sleep 5
STATUS=$(n8n exec get "$EXEC_ID" -o json | jq -r '.status')

if [ "$STATUS" = "success" ]; then
  echo "Deployment test passed"
  exit 0
else
  echo "Deployment test failed"
  n8n exec get "$EXEC_ID" --include-data
  exit 1
fi
```

### Scheduled Trigger via Cron

```bash
# crontab entry to run workflow every hour
0 * * * * /usr/local/bin/n8n exec run wf_abc123 -d '{"source": "cron"}' >> /var/log/n8n-cron.log 2>&1
```

### Error Handling

```bash
#!/bin/bash

WORKFLOW="wf_abc123"
DATA='{"important": "data"}'

# Run with error handling
if ! RESULT=$(n8n exec run "$WORKFLOW" -d "$DATA" -o json 2>&1); then
  echo "Failed to start execution: $RESULT"
  exit 1
fi

EXEC_ID=$(echo "$RESULT" | jq -r '.id')

# Monitor with timeout
TIMEOUT=60
ELAPSED=0

while [ $ELAPSED -lt $TIMEOUT ]; do
  STATUS=$(n8n exec get "$EXEC_ID" -o json | jq -r '.status')

  case "$STATUS" in
    success)
      echo "Execution completed successfully"
      exit 0
      ;;
    error)
      echo "Execution failed"
      n8n exec get "$EXEC_ID" --include-data -o json | jq '.data'
      exit 1
      ;;
    *)
      sleep 5
      ELAPSED=$((ELAPSED + 5))
      ;;
  esac
done

echo "Execution timed out after ${TIMEOUT}s"
exit 1
```

---

## Comparison Summary

| Feature | `workflows run` | `executions run` |
|---------|----------------|------------------|
| Trigger type | Webhook/Form only | Any |
| Method | HTTP to webhook URL | n8n API |
| Workflow state | Must be active | Can be inactive |
| HTTP method | Configurable | N/A |
| Wait option | `--no-wait` | `--wait` |
| Best for | Testing webhooks | General execution |

---

## See Also

- [Workflows Commands](../commands/workflows.md) - Full workflow command reference
- [Executions Commands](../commands/executions.md) - Execution management
- [Scripting Guide](./scripting.md) - More automation patterns
