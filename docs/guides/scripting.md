# Scripting & Automation

Use the n8n CLI in shell scripts, CI/CD pipelines, and automated workflows.

## JSON Output

For scripting, use JSON output format for reliable parsing:

```bash
n8n workflows list -o json
n8n workflows list -o json-pretty  # Human-readable
```

### Parsing with jq

```bash
# Get workflow IDs
n8n wf list -o json | jq -r '.[].id'

# Get active workflow names
n8n wf list --active true -o json | jq -r '.[].name'

# Count workflows
n8n wf list -o json | jq length

# Filter locally
n8n wf list -o json | jq '.[] | select(.name | contains("sync"))'
```

## Exit Codes

The CLI uses standard Unix exit codes for scripting:

| Code | Constant | Meaning |
|------|----------|---------|
| 0 | `EX_OK` | Success |
| 1 | `EX_ERROR` | General error |
| 65 | `EX_DATAERR` | Data format error (invalid JSON, parse failure) |
| 69 | `EX_UNAVAILABLE` | Resource not found (404) |
| 74 | `EX_IOERR` | I/O error (file read/write) |
| 77 | `EX_NOPERM` | Permission denied (401/403) |
| 78 | `EX_CONFIG` | Configuration error |
| 130 | `EX_CANCELLED` | User interrupted (Ctrl+C) |

### Using Exit Codes

```bash
#!/bin/bash

if n8n health check; then
  echo "n8n is healthy"
else
  case $? in
    69) echo "n8n not reachable" ;;
    77) echo "Authentication failed" ;;
    78) echo "Configuration error" ;;
    *)  echo "Unknown error" ;;
  esac
  exit 1
fi
```

## Quiet Mode

Suppress status messages for cleaner output:

```bash
# Only output data, no status messages
n8n -q workflows list -o json

# Useful in scripts
WORKFLOWS=$(n8n -q wf list -o json)
```

## Environment Variables

Configure via environment for CI/CD:

```bash
export N8N_BASE_URL="https://n8n.example.com"
export N8N_API_KEY="your-api-key"

# All commands use these values
n8n wf list
n8n exec list
```

## Common Patterns

### Check if Workflow Exists

```bash
#!/bin/bash

workflow_exists() {
  n8n wf get "$1" >/dev/null 2>&1
}

if workflow_exists "wf_abc123"; then
  echo "Workflow exists"
else
  echo "Workflow not found"
fi
```

### Get Workflow by Name

```bash
#!/bin/bash

get_workflow_id() {
  local name="$1"
  n8n wf list -o json | jq -r ".[] | select(.name == \"$name\") | .id"
}

WF_ID=$(get_workflow_id "My Workflow")
if [ -n "$WF_ID" ]; then
  echo "Found: $WF_ID"
fi
```

### Wait for Execution

```bash
#!/bin/bash

wait_for_execution() {
  local exec_id="$1"
  local timeout="${2:-300}"
  local elapsed=0

  while [ $elapsed -lt $timeout ]; do
    local status=$(n8n exec get "$exec_id" -o json | jq -r '.status')

    case "$status" in
      success) return 0 ;;
      error)   return 1 ;;
      *)       sleep 5; elapsed=$((elapsed + 5)) ;;
    esac
  done

  return 2  # Timeout
}

# Usage
EXEC_ID=$(n8n exec run wf_abc123 -o json | jq -r '.id')
if wait_for_execution "$EXEC_ID" 60; then
  echo "Success"
else
  echo "Failed or timed out"
fi
```

### Retry with Backoff

```bash
#!/bin/bash

retry_with_backoff() {
  local max_attempts=5
  local delay=1
  local attempt=1

  while [ $attempt -le $max_attempts ]; do
    if "$@"; then
      return 0
    fi

    echo "Attempt $attempt failed, retrying in ${delay}s..."
    sleep $delay
    delay=$((delay * 2))
    attempt=$((attempt + 1))
  done

  return 1
}

# Usage
retry_with_backoff n8n wf activate wf_abc123
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Deploy Workflows

on:
  push:
    branches: [main]
    paths:
      - 'workflows/**'

env:
  N8N_BASE_URL: ${{ secrets.N8N_URL }}
  N8N_API_KEY: ${{ secrets.N8N_API_KEY }}

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Build CLI
        run: cargo build --release

      - name: Add to PATH
        run: echo "$PWD/target/release" >> $GITHUB_PATH

      - name: Check n8n Health
        run: n8n health ready

      - name: Deploy Workflows
        run: |
          for file in workflows/*.json; do
            name=$(basename "$file" .json)
            echo "Deploying: $name"

            # Get existing workflow ID or create new
            WF_ID=$(n8n wf list -o json | jq -r ".[] | select(.name == \"$name\") | .id")

            if [ -n "$WF_ID" ]; then
              n8n wf update "$WF_ID" "$file"
            else
              n8n wf create "$file"
            fi
          done

      - name: Activate Production Workflows
        run: |
          n8n wf list -t production -o json | jq -r '.[].id' | while read id; do
            n8n wf activate "$id"
          done
```

### GitLab CI

```yaml
stages:
  - test
  - deploy

variables:
  N8N_BASE_URL: ${N8N_URL}
  N8N_API_KEY: ${N8N_API_KEY}

deploy-workflows:
  stage: deploy
  image: rust:latest
  script:
    - cargo build --release
    - export PATH="$PWD/target/release:$PATH"
    - n8n health check
    - |
      for file in workflows/*.json; do
        n8n wf update "$(jq -r .id "$file")" "$file"
      done
  only:
    - main
  when: manual
```

### Jenkins

```groovy
pipeline {
  agent any

  environment {
    N8N_BASE_URL = credentials('n8n-url')
    N8N_API_KEY = credentials('n8n-api-key')
  }

  stages {
    stage('Build CLI') {
      steps {
        sh 'cargo build --release'
      }
    }

    stage('Deploy') {
      steps {
        sh '''
          export PATH="$PWD/target/release:$PATH"
          n8n health check
          n8n wf update wf_abc123 workflow.json
          n8n wf activate wf_abc123
        '''
      }
    }
  }
}
```

## Backup & Restore

### Backup All Workflows

```bash
#!/bin/bash

BACKUP_DIR="./backups/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"

echo "Backing up workflows to $BACKUP_DIR"

n8n wf list -o json | jq -r '.[].id' | while read id; do
  name=$(n8n wf get "$id" -o json | jq -r '.name' | tr ' ' '_')
  echo "Exporting: $name"
  n8n wf export "$id" --pretty --file "$BACKUP_DIR/${name}.json"
done

echo "Backup complete: $(ls "$BACKUP_DIR" | wc -l) workflows"
```

### Restore Workflows

```bash
#!/bin/bash

BACKUP_DIR="$1"

if [ -z "$BACKUP_DIR" ]; then
  echo "Usage: restore.sh <backup_dir>"
  exit 1
fi

for file in "$BACKUP_DIR"/*.json; do
  name=$(jq -r '.name' "$file")
  echo "Restoring: $name"

  # Check if exists
  WF_ID=$(n8n wf list -o json | jq -r ".[] | select(.name == \"$name\") | .id")

  if [ -n "$WF_ID" ]; then
    n8n wf update "$WF_ID" "$file"
  else
    n8n wf create "$file"
  fi
done
```

## Monitoring Scripts

### Health Monitor

```bash
#!/bin/bash

LOG_FILE="/var/log/n8n-health.log"
ALERT_EMAIL="admin@example.com"

check_health() {
  local timestamp=$(date '+%Y-%m-%d %H:%M:%S')

  if n8n health ready 2>/dev/null; then
    echo "$timestamp OK" >> "$LOG_FILE"
    return 0
  else
    echo "$timestamp FAILED" >> "$LOG_FILE"
    return 1
  fi
}

if ! check_health; then
  echo "n8n health check failed at $(date)" | \
    mail -s "n8n Alert" "$ALERT_EMAIL"
fi
```

### Execution Monitor

```bash
#!/bin/bash

# Alert on failed executions

WORKFLOW_ID="$1"
LOOKBACK_MINUTES=60

# Get recent failures
FAILURES=$(n8n exec list -w "$WORKFLOW_ID" -s error -o json | \
  jq --arg cutoff "$(date -v-${LOOKBACK_MINUTES}M -Iseconds)" \
  '[.[] | select(.startedAt > $cutoff)] | length')

if [ "$FAILURES" -gt 0 ]; then
  echo "Alert: $FAILURES failures in last $LOOKBACK_MINUTES minutes"
  echo "Workflow: $WORKFLOW_ID"
  n8n exec list -w "$WORKFLOW_ID" -s error --limit 5
fi
```

### Active Workflow Report

```bash
#!/bin/bash

echo "=== n8n Workflow Report ==="
echo "Generated: $(date)"
echo ""

echo "Active Workflows:"
n8n wf list --active true -o json | \
  jq -r '.[] | "  - \(.name) (\(.id))"'

echo ""
echo "Inactive Workflows:"
n8n wf list --active false -o json | \
  jq -r '.[] | "  - \(.name) (\(.id))"'

echo ""
echo "Summary:"
echo "  Total: $(n8n wf list -o json | jq length)"
echo "  Active: $(n8n wf list --active true -o json | jq length)"
echo "  Inactive: $(n8n wf list --active false -o json | jq length)"
```

## Bulk Operations

### Activate All Workflows

```bash
n8n wf list --active false -o json | jq -r '.[].id' | while read id; do
  echo "Activating: $id"
  n8n wf activate "$id"
done
```

### Deactivate by Tag

```bash
n8n wf list -t deprecated -o json | jq -r '.[].id' | while read id; do
  echo "Deactivating: $id"
  n8n wf deactivate "$id"
done
```

### Delete Old Executions

```bash
#!/bin/bash

# Delete executions older than 30 days
CUTOFF=$(date -v-30d -Iseconds)

n8n exec list -o json | \
  jq -r --arg cutoff "$CUTOFF" '.[] | select(.startedAt < $cutoff) | .id' | \
  while read id; do
    echo "Deleting execution: $id"
    n8n exec delete "$id"
  done
```

### Tag Multiple Workflows

```bash
#!/bin/bash

TAG_ID="tag_production"
WORKFLOWS=("wf_abc123" "wf_def456" "wf_ghi789")

for wf in "${WORKFLOWS[@]}"; do
  echo "Tagging: $wf"
  n8n tags assign "$wf" -t "$TAG_ID"
done
```

## Docker Integration

### Run CLI in Docker

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/n8n /usr/local/bin/
ENTRYPOINT ["n8n"]
```

### Docker Compose Job

```yaml
version: '3.8'
services:
  n8n:
    image: n8nio/n8n
    # ... n8n config

  cli-job:
    build: ./n8n-cli
    environment:
      - N8N_BASE_URL=http://n8n:5678
      - N8N_API_KEY=${N8N_API_KEY}
    depends_on:
      - n8n
    command: workflows list
```

## Error Handling

### Comprehensive Error Handler

```bash
#!/bin/bash
set -euo pipefail

handle_error() {
  local exit_code=$?
  local line_no=$1

  case $exit_code in
    65) echo "Error: Invalid data format (line $line_no)" ;;
    69) echo "Error: Resource not found (line $line_no)" ;;
    77) echo "Error: Authentication failed (line $line_no)" ;;
    78) echo "Error: Configuration error (line $line_no)" ;;
    *)  echo "Error: Unknown error $exit_code (line $line_no)" ;;
  esac

  exit $exit_code
}

trap 'handle_error $LINENO' ERR

# Your script here
n8n health check
n8n wf list
```

---

## See Also

- [Commands Overview](../commands/README.md) - Exit codes and global flags
- [Configuration](../configuration.md) - Environment variables
- [Error Handling](../development/error-handling.md) - Complete error reference
