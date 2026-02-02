# Tags Commands

Organize workflows with tags for easier filtering and management.

## Commands

- [list](#list) - List all tags
- [create](#create) - Create a tag
- [update](#update) - Update a tag
- [delete](#delete) - Delete a tag
- [assign](#assign) - Assign tags to a workflow

---

## list

List all tags in the n8n instance.

```bash
n8n tags list
```

### Examples

```bash
# List all tags
n8n tags list

# Output as JSON
n8n tags list -o json
```

### Output Columns

| Column | Description |
|--------|-------------|
| ID | Tag ID |
| Name | Tag name |
| Created | Creation timestamp |
| Updated | Last update timestamp |

---

## create

Create a new tag.

```bash
n8n tags create --name <NAME>
```

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--name <NAME>` | `-n` | Tag name (required) |

### Examples

```bash
# Create a tag
n8n tags create -n "production"

# Create multiple tags
n8n tags create -n "staging"
n8n tags create -n "critical"
n8n tags create -n "data-sync"
```

### Naming Conventions

Consider using consistent naming patterns:

| Pattern | Example | Use Case |
|---------|---------|----------|
| Environment | `production`, `staging`, `dev` | Deployment stage |
| Priority | `critical`, `high`, `low` | Importance level |
| Category | `data-sync`, `notifications`, `reports` | Workflow purpose |
| Team | `team-a`, `marketing`, `engineering` | Ownership |

---

## update

Update a tag's name.

```bash
n8n tags update <ID> --name <NAME>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Tag ID to update |

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--name <NAME>` | `-n` | New tag name (required) |

### Examples

```bash
# Rename a tag
n8n tags update tag_abc123 -n "prod"
```

---

## delete

Delete a tag.

```bash
n8n tags delete <ID>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Tag ID to delete |

### Examples

```bash
n8n tags delete tag_abc123
```

### Notes

- Deleting a tag removes it from all workflows
- Workflows are not deleted, only the tag association

---

## assign

Assign tags to a workflow.

```bash
n8n tags assign <WORKFLOW_ID> --tags <TAG_IDS>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `WORKFLOW_ID` | Workflow to tag |

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--tags <IDS>` | `-t` | Comma-separated tag IDs |

### Examples

```bash
# Assign single tag
n8n tags assign wf_abc123 -t tag_xyz

# Assign multiple tags
n8n tags assign wf_abc123 -t tag_prod,tag_critical,tag_sync

# Clear all tags (assign empty)
n8n tags assign wf_abc123 -t ""
```

### Important

- This **replaces** all existing tags on the workflow
- To add a tag without removing others, include all desired tag IDs
- Use `n8n workflows get <id>` to see current tags

---

## Workflows by Tag

List workflows filtered by tag using the workflows command:

```bash
# Filter workflows by tag
n8n wf list --tags production

# Multiple tags (comma-separated)
n8n wf list -t production,critical
```

---

## Scripting Examples

### Create Standard Tags

```bash
#!/bin/bash
# Initialize standard tags for a new n8n instance

TAGS=("production" "staging" "development" "critical" "data-sync" "notifications" "reports")

for tag in "${TAGS[@]}"; do
  echo "Creating tag: $tag"
  n8n tags create -n "$tag"
done
```

### Tag All Active Workflows

```bash
#!/bin/bash
# Tag all active workflows as "production"

# First, get or create the production tag
PROD_TAG=$(n8n tags list -o json | jq -r '.[] | select(.name=="production") | .id')

if [ -z "$PROD_TAG" ]; then
  n8n tags create -n "production"
  PROD_TAG=$(n8n tags list -o json | jq -r '.[] | select(.name=="production") | .id')
fi

# Tag all active workflows
n8n wf list --active true -o json | \
  jq -r '.[].id' | \
  while read wf_id; do
    echo "Tagging workflow: $wf_id"
    n8n tags assign "$wf_id" -t "$PROD_TAG"
  done
```

### List Workflows Without Tags

```bash
#!/bin/bash
# Find workflows that haven't been tagged

n8n wf list -o json | \
  jq -r '.[] | select(.tags | length == 0) | "\(.id)\t\(.name)"' | \
  while IFS=$'\t' read id name; do
    echo "Untagged: $name ($id)"
  done
```

### Generate Tag Report

```bash
#!/bin/bash
# Count workflows per tag

echo "Workflow count by tag:"
echo "======================"

n8n tags list -o json | jq -r '.[].name' | while read tag; do
  COUNT=$(n8n wf list -t "$tag" -o json | jq length)
  printf "%-20s %d\n" "$tag" "$COUNT"
done
```

---

## Best Practices

### 1. Consistent Naming

Use a consistent naming convention across your organization:

```
environment:production
environment:staging
team:engineering
team:marketing
priority:critical
type:data-sync
```

### 2. Avoid Tag Proliferation

- Review tags periodically
- Delete unused tags
- Consolidate similar tags

### 3. Document Tag Meanings

Maintain documentation of what each tag means:

| Tag | Meaning | Applied To |
|-----|---------|------------|
| `production` | Live in production | Active, tested workflows |
| `critical` | High priority | Business-critical workflows |
| `deprecated` | Scheduled for removal | Old workflows |

### 4. Use Tags for Filtering

Tags are most useful when used consistently for filtering:

```bash
# Production workflows only
n8n wf list -t production

# Critical production workflows
n8n wf list -t production,critical

# Non-production
n8n wf list -t staging,development
```

---

## See Also

- [Workflows Commands](./workflows.md) - Filter workflows by tags
- [Scripting Guide](../guides/scripting.md) - Automation examples
