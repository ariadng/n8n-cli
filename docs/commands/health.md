# Health Commands

Check the health and readiness of your n8n instance.

## Commands

- [check](#check) - Basic health check
- [ready](#ready) - Readiness check

---

## check

Perform a basic health check on the n8n instance.

```bash
n8n health check
```

### Description

Calls the `/healthz` endpoint to verify the n8n instance is running and responsive.

### Examples

```bash
# Simple health check
n8n health check

# Check different instances
n8n -p production health check
n8n -p staging health check

# Use in scripts (check exit code)
if n8n health check; then
  echo "n8n is healthy"
else
  echo "n8n is not responding"
fi
```

### Output

Success:
```
Health check: OK
```

Failure:
```
Error: Connection failed: Connection refused
```

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Health check passed |
| 69 | Service unavailable |
| 1 | Other error |

---

## ready

Check if the n8n instance is ready to accept requests.

```bash
n8n health ready
```

### Description

Calls the `/healthz/readiness` endpoint to verify the n8n instance is fully initialized and ready to handle workflow executions.

### Difference from `check`

| Check | Purpose |
|-------|---------|
| `health check` | Is the process running? |
| `health ready` | Is it ready to serve requests? |

The readiness check may fail during:
- Initial startup (database migrations)
- Resource constraints
- Dependency failures

### Examples

```bash
# Readiness check
n8n health ready

# Wait for readiness (startup scripts)
while ! n8n health ready 2>/dev/null; do
  echo "Waiting for n8n to be ready..."
  sleep 5
done
echo "n8n is ready!"
```

### Output

Success:
```
Readiness check: OK
```

Failure:
```
Error: Service not ready
```

---

## Scripting Examples

### Startup Wait Script

```bash
#!/bin/bash
# Wait for n8n to be ready before proceeding

MAX_ATTEMPTS=30
ATTEMPT=1

echo "Waiting for n8n to be ready..."

while [ $ATTEMPT -le $MAX_ATTEMPTS ]; do
  if n8n health ready 2>/dev/null; then
    echo "n8n is ready!"
    exit 0
  fi

  echo "Attempt $ATTEMPT/$MAX_ATTEMPTS - not ready yet..."
  ATTEMPT=$((ATTEMPT + 1))
  sleep 2
done

echo "Error: n8n did not become ready after $MAX_ATTEMPTS attempts"
exit 1
```

### Monitoring Script

```bash
#!/bin/bash
# Simple monitoring script

LOG_FILE="/var/log/n8n-health.log"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

if n8n health check 2>/dev/null; then
  echo "$TIMESTAMP - OK" >> "$LOG_FILE"
else
  echo "$TIMESTAMP - FAILED" >> "$LOG_FILE"

  # Alert (customize as needed)
  echo "n8n health check failed at $TIMESTAMP" | \
    mail -s "n8n Alert" admin@example.com
fi
```

### Multi-Instance Health Check

```bash
#!/bin/bash
# Check health of multiple n8n instances

INSTANCES=("production" "staging" "development")

echo "n8n Health Status"
echo "================="

for instance in "${INSTANCES[@]}"; do
  if n8n -p "$instance" health check 2>/dev/null; then
    STATUS="OK"
  else
    STATUS="FAILED"
  fi
  printf "%-15s %s\n" "$instance:" "$STATUS"
done
```

### Kubernetes Probes

Example Kubernetes deployment using health checks:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: n8n-cli-job
spec:
  template:
    spec:
      containers:
        - name: cli
          image: your-n8n-cli-image
          env:
            - name: N8N_BASE_URL
              value: "http://n8n-service:5678"
            - name: N8N_API_KEY
              valueFrom:
                secretKeyRef:
                  name: n8n-secrets
                  key: api-key
          # Wait for n8n before running jobs
          command:
            - /bin/sh
            - -c
            - |
              while ! n8n health ready; do
                sleep 5
              done
              # Run your CLI commands here
              n8n workflows list
```

### Docker Compose Health Check

```yaml
version: '3.8'
services:
  n8n:
    image: n8nio/n8n
    healthcheck:
      test: ["CMD", "wget", "-q", "--spider", "http://localhost:5678/healthz"]
      interval: 30s
      timeout: 10s
      retries: 3

  cli-jobs:
    image: your-n8n-cli-image
    depends_on:
      n8n:
        condition: service_healthy
    environment:
      - N8N_BASE_URL=http://n8n:5678
      - N8N_API_KEY=${N8N_API_KEY}
    command: n8n workflows list
```

---

## CI/CD Integration

### Pre-Deployment Check

```bash
#!/bin/bash
# Verify n8n is healthy before deploying workflows

if ! n8n -p production health ready; then
  echo "Error: Production n8n is not ready"
  exit 1
fi

echo "Production n8n is healthy, proceeding with deployment..."
n8n -p production workflows update wf_123 workflow.json
n8n -p production workflows activate wf_123
```

### GitHub Actions

```yaml
name: Deploy Workflow

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install n8n CLI
        run: cargo install --path .

      - name: Check n8n Health
        env:
          N8N_BASE_URL: ${{ secrets.N8N_URL }}
          N8N_API_KEY: ${{ secrets.N8N_API_KEY }}
        run: n8n health ready

      - name: Deploy Workflow
        env:
          N8N_BASE_URL: ${{ secrets.N8N_URL }}
          N8N_API_KEY: ${{ secrets.N8N_API_KEY }}
        run: |
          n8n workflows update ${{ vars.WORKFLOW_ID }} workflow.json
          n8n workflows activate ${{ vars.WORKFLOW_ID }}
```

---

## See Also

- [Configuration](../configuration.md) - Configure n8n connection settings
- [Scripting Guide](../guides/scripting.md) - More automation examples
