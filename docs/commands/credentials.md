# Credentials Commands

Manage n8n credentials for authenticating with external services.

**Alias:** `cred`

## Commands

- [list](#list) - List credentials
- [schema](#schema) - Get credential type schema
- [create](#create) - Create a credential
- [update](#update) - Update a credential
- [delete](#delete) - Delete a credential

---

## list

List all credentials, optionally filtered by type.

```bash
n8n credentials list [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--type <TYPE>` | Filter by credential type |

### Examples

```bash
# List all credentials
n8n cred list

# Filter by type
n8n cred list --type httpBasicAuth

# Output as JSON
n8n cred list -o json
```

### Output Columns

| Column | Description |
|--------|-------------|
| ID | Credential ID |
| Name | Display name |
| Type | Credential type (e.g., `httpBasicAuth`, `oAuth2Api`) |
| Created | Creation timestamp |
| Updated | Last update timestamp |

### Common Credential Types

| Type | Description |
|------|-------------|
| `httpBasicAuth` | HTTP Basic Authentication |
| `httpHeaderAuth` | HTTP Header Authentication |
| `oAuth2Api` | OAuth 2.0 |
| `apiKey` | API Key |
| `slackApi` | Slack API |
| `githubApi` | GitHub API |
| `googleApi` | Google API |

---

## schema

Get the schema for a credential type, showing required and optional fields.

```bash
n8n credentials schema <TYPE>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `TYPE` | Credential type name |

### Examples

```bash
# Get HTTP Basic Auth schema
n8n cred schema httpBasicAuth

# Get OAuth2 schema as JSON
n8n cred schema oAuth2Api -o json-pretty

# Get Slack API schema
n8n cred schema slackApi
```

### Output

The schema shows:
- Required fields and their types
- Optional fields
- Field descriptions
- Default values
- Validation rules

This is useful for knowing what data to provide when creating credentials.

---

## create

Create a new credential.

```bash
n8n credentials create [OPTIONS]
```

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--name <NAME>` | `-n` | Credential display name (required) |
| `--type <TYPE>` | `-t` | Credential type (required) |
| `--data <JSON>` | `-d` | Credential data as JSON (required) |

### Examples

```bash
# Create HTTP Basic Auth credential
n8n cred create \
  -n "My API Credentials" \
  -t httpBasicAuth \
  -d '{"user": "apiuser", "password": "secret123"}'

# Create API Key credential
n8n cred create \
  -n "Service API Key" \
  -t httpHeaderAuth \
  -d '{"name": "X-API-Key", "value": "abc123xyz"}'

# Create with complex data
n8n cred create \
  -n "OAuth Credentials" \
  -t oAuth2Api \
  -d '{
    "clientId": "your-client-id",
    "clientSecret": "your-client-secret",
    "accessTokenUrl": "https://oauth.example.com/token",
    "authUrl": "https://oauth.example.com/authorize"
  }'
```

### Security Notes

- Credential data is encrypted at rest by n8n
- The CLI transmits data over HTTPS (if configured)
- Avoid storing credentials in shell history:

```bash
# Use a file instead
echo '{"user": "admin", "password": "secret"}' > /tmp/cred.json
n8n cred create -n "My Cred" -t httpBasicAuth -d "$(cat /tmp/cred.json)"
rm /tmp/cred.json

# Or use environment variables
n8n cred create -n "My Cred" -t httpBasicAuth \
  -d "{\"user\": \"$API_USER\", \"password\": \"$API_PASSWORD\"}"
```

---

## update

Update an existing credential.

```bash
n8n credentials update <ID> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Credential ID to update |

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--name <NAME>` | `-n` | New display name |
| `--data <JSON>` | `-d` | New credential data |

### Examples

```bash
# Update credential name
n8n cred update cred_abc123 -n "Production API Key"

# Update credential data
n8n cred update cred_abc123 -d '{"user": "newuser", "password": "newpassword"}'

# Update both
n8n cred update cred_abc123 \
  -n "Updated Credentials" \
  -d '{"apiKey": "new-api-key-value"}'
```

### Important Notes

- Updating credential data **replaces** all data, not just specified fields
- Always provide the complete credential data when updating
- The credential type cannot be changed after creation

---

## delete

Delete a credential.

```bash
n8n credentials delete <ID>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `ID` | Credential ID to delete |

### Examples

```bash
n8n cred delete cred_abc123
```

### Warnings

- Deleting a credential may break workflows that use it
- Check for dependent workflows before deleting
- This action cannot be undone

---

## Scripting Examples

### Backup All Credentials

```bash
#!/bin/bash
# Export credential metadata (not secret values)

n8n cred list -o json-pretty > credentials-backup.json
echo "Backed up credential list to credentials-backup.json"
```

### Check for Unused Credentials

```bash
#!/bin/bash
# List credentials and check usage (manual review needed)

echo "Credentials in system:"
n8n cred list

echo ""
echo "Review each credential to ensure it's still needed."
```

### Rotate Credentials

```bash
#!/bin/bash
# Update credential with new values

CRED_ID="cred_abc123"
NEW_PASSWORD="$(openssl rand -base64 32)"

n8n cred update "$CRED_ID" -d "{\"user\": \"apiuser\", \"password\": \"$NEW_PASSWORD\"}"

echo "Credential rotated. New password: $NEW_PASSWORD"
echo "Store this password securely!"
```

---

## See Also

- [Workflows Commands](./workflows.md) - Workflows use credentials for external services
- [Configuration](../configuration.md) - CLI configuration including API key management
