# Error Handling

Comprehensive guide to error types, exit codes, and error handling patterns.

## Overview

The n8n CLI uses a unified error type `N8nError` that:

- Provides user-friendly messages
- Maps to Unix exit codes
- Preserves error context via source errors

## N8nError Enum

```rust
#[derive(Debug, Error)]
pub enum N8nError {
    // Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid API key format")]
    InvalidApiKey,

    #[error("API key not configured. Set N8N_API_KEY environment variable or use --api-key")]
    MissingApiKey,

    #[error("Base URL not configured. Set N8N_BASE_URL environment variable or use --url")]
    MissingBaseUrl,

    #[error("Profile '{0}' not found in configuration")]
    ProfileNotFound(String),

    #[error("Failed to read config file: {0}")]
    ConfigFileRead(#[source] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    ConfigFileParse(#[source] toml::de::Error),

    // HTTP/Network errors
    #[error("HTTP client error: {0}")]
    HttpClient(#[source] reqwest::Error),

    #[error("Request failed: {0}")]
    Request(#[source] reqwest::Error),

    #[error("Connection failed to {url}: {message}\n\nHint: Check that n8n is running and accessible")]
    ConnectionFailed { url: String, message: String },

    // API errors
    #[error("{}", .0.user_message())]
    Api(ApiErrorResponse),

    // Serialization errors
    #[error("Failed to parse response: {0}")]
    Deserialize(#[source] reqwest::Error),

    #[error("Failed to parse input: {0}")]
    InvalidInput(#[source] serde_json::Error),

    #[error("Failed to serialize data: {0}")]
    Serialize(#[source] serde_json::Error),

    // Resource errors
    #[error("Workflow '{0}' not found")]
    WorkflowNotFound(String),

    #[error("Execution '{0}' not found")]
    ExecutionNotFound(String),

    #[error("Credential '{0}' not found")]
    CredentialNotFound(String),

    #[error("Node '{0}' not found in workflow")]
    NodeNotFound(String),

    #[error("Connection not found: {from} -> {to}")]
    ConnectionNotFound { from: String, to: String },

    // I/O errors
    #[error("Failed to read file '{path}': {source}")]
    FileRead { path: String, #[source] source: std::io::Error },

    #[error("Failed to write file '{path}': {source}")]
    FileWrite { path: String, #[source] source: std::io::Error },

    #[error("Failed to read from stdin: {0}")]
    StdinRead(#[source] std::io::Error),

    // User interaction
    #[error("Operation cancelled by user")]
    Cancelled,

    // Workflow editing
    #[error("Validation failed:\n{0}")]
    ValidationFailed(String),

    #[error("Editor failed: {0}")]
    EditorFailed(String),

    #[error("No changes detected")]
    NoChanges,
}
```

## Exit Codes

Unix-standard exit codes for scripting:

| Code | Constant | Meaning | Error Types |
|------|----------|---------|-------------|
| 0 | `EX_OK` | Success | `NoChanges` |
| 1 | `EX_ERROR` | General error | Generic `Api` errors |
| 65 | `EX_DATAERR` | Data format error | `InvalidInput`, `Serialize`, `Deserialize`, `ValidationFailed` |
| 69 | `EX_UNAVAILABLE` | Resource unavailable | `WorkflowNotFound`, `ExecutionNotFound`, `CredentialNotFound`, `NodeNotFound`, `ConnectionNotFound`, `ConnectionFailed`, `Request`, `HttpClient`, API 404 |
| 74 | `EX_IOERR` | I/O error | `FileRead`, `FileWrite`, `StdinRead`, `EditorFailed` |
| 77 | `EX_NOPERM` | Permission denied | `InvalidApiKey`, `MissingApiKey`, API 401/403 |
| 78 | `EX_CONFIG` | Configuration error | `Config`, `ProfileNotFound`, `ConfigFileRead`, `ConfigFileParse`, `MissingBaseUrl` |
| 130 | - | Cancelled | `Cancelled` (Ctrl+C) |

### Exit Code Implementation

```rust
impl N8nError {
    pub fn exit_code(&self) -> i32 {
        match self {
            // Configuration errors (EX_CONFIG = 78)
            Self::Config(_)
            | Self::ProfileNotFound(_)
            | Self::ConfigFileRead(_)
            | Self::ConfigFileParse(_)
            | Self::MissingBaseUrl => 78,

            // Permission errors (EX_NOPERM = 77)
            Self::InvalidApiKey | Self::MissingApiKey => 77,
            Self::Api(e) if e.code == 401 || e.code == 403 => 77,

            // Resource unavailable (EX_UNAVAILABLE = 69)
            Self::Api(e) if e.code == 404 => 69,
            Self::WorkflowNotFound(_)
            | Self::ExecutionNotFound(_)
            | Self::CredentialNotFound(_) => 69,
            Self::ConnectionFailed { .. } | Self::Request(_) | Self::HttpClient(_) => 69,
            Self::NodeNotFound(_) | Self::ConnectionNotFound { .. } => 69,

            // I/O errors (EX_IOERR = 74)
            Self::FileRead { .. } | Self::FileWrite { .. } | Self::StdinRead(_) => 74,
            Self::EditorFailed(_) => 74,

            // Data errors (EX_DATAERR = 65)
            Self::InvalidInput(_) | Self::Serialize(_) | Self::Deserialize(_) => 65,
            Self::ValidationFailed(_) => 65,

            // User cancelled
            Self::Cancelled => 130,

            // No changes (not an error)
            Self::NoChanges => 0,

            // Generic failure
            Self::Api(_) => 1,
        }
    }
}
```

## API Error Response

```rust
#[derive(Debug, Deserialize, Clone)]
pub struct ApiErrorResponse {
    pub code: u16,
    pub message: String,
    pub hint: Option<String>,
}

impl ApiErrorResponse {
    /// Format error for user display
    pub fn user_message(&self) -> String {
        match &self.hint {
            Some(hint) => format!("{}\n\nHint: {}", self.message, hint),
            None => self.message.clone(),
        }
    }

    /// Create a generic error for unknown status codes
    pub fn unknown(status: reqwest::StatusCode) -> Self {
        Self {
            code: status.as_u16(),
            message: format!("Request failed with status {}", status),
            hint: None,
        }
    }
}
```

## Result Type

```rust
pub type Result<T> = std::result::Result<T, N8nError>;
```

## Error Handling Patterns

### Propagating Errors

```rust
// Use ? operator for clean propagation
async fn get_workflow(client: &N8nClient, id: &str) -> Result<WorkflowDetail> {
    let workflow = client.get_workflow(id).await?;
    Ok(workflow)
}
```

### Adding Context

```rust
// Wrap errors with more specific types
let workflow = client.get_workflow(&id).await
    .map_err(|e| match e {
        N8nError::Api(ref api) if api.code == 404 => {
            N8nError::WorkflowNotFound(id.clone())
        }
        _ => e,
    })?;
```

### File Operations

```rust
// Include path in error
let content = std::fs::read_to_string(&path)
    .map_err(|e| N8nError::FileRead {
        path: path.display().to_string(),
        source: e,
    })?;
```

### User-Friendly Messages

```rust
// Create errors with helpful hints
N8nError::Config(
    "Workflow has no webhook trigger. Only webhook workflows can be run via CLI.\n\
     Hint: Add a Webhook or Form Trigger node, or run manually in n8n UI."
        .to_string(),
)
```

## Main Error Handling

```rust
#[tokio::main]
async fn main() {
    let result = run().await;

    match result {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            // Print error message to stderr
            eprintln!("Error: {e}");

            // Exit with appropriate code
            std::process::exit(e.exit_code());
        }
    }
}
```

## Adding New Error Types

### 1. Add Variant

```rust
#[derive(Debug, Error)]
pub enum N8nError {
    // ... existing variants ...

    #[error("Variable '{0}' not found")]
    VariableNotFound(String),
}
```

### 2. Assign Exit Code

```rust
impl N8nError {
    pub fn exit_code(&self) -> i32 {
        match self {
            // ... existing matches ...
            Self::VariableNotFound(_) => 69,  // EX_UNAVAILABLE
        }
    }
}
```

### 3. Use in Code

```rust
let variable = client.get_variable(&key).await
    .map_err(|_| N8nError::VariableNotFound(key.clone()))?;
```

## Error Categories

### Configuration Errors (78)

Problems with CLI configuration:
- Missing required config values
- Invalid config file format
- Unknown profile

User action: Check config file or environment variables.

### Permission Errors (77)

Authentication/authorization failures:
- Invalid API key
- Missing API key
- API returns 401/403

User action: Verify API key is correct and has necessary permissions.

### Resource Unavailable (69)

Requested resource doesn't exist or isn't reachable:
- Workflow/execution/credential not found
- Connection failed
- Network timeout

User action: Verify resource exists and n8n is accessible.

### I/O Errors (74)

File system or editor problems:
- Can't read input file
- Can't write output file
- Editor process failed

User action: Check file permissions and paths.

### Data Errors (65)

Invalid data format:
- Malformed JSON input
- Response parsing failure
- Validation errors

User action: Check input data format.

### Cancelled (130)

User interrupted operation:
- Ctrl+C pressed
- Declined confirmation prompt

User action: Intentional, no action needed.

## Best Practices

### 1. Use Specific Error Types

```rust
// Good: Specific error type
N8nError::WorkflowNotFound(id.clone())

// Avoid: Generic config error for specific issues
N8nError::Config("Workflow not found".to_string())
```

### 2. Include Helpful Hints

```rust
N8nError::ConnectionFailed {
    url: url.to_string(),
    message: "Connection refused".to_string(),
}
// Output: Connection failed to https://...: Connection refused
//
// Hint: Check that n8n is running and accessible
```

### 3. Preserve Source Errors

```rust
#[error("Failed to read file '{path}': {source}")]
FileRead {
    path: String,
    #[source]
    source: std::io::Error,
}
```

### 4. Use Appropriate Exit Codes

Scripts rely on exit codes for control flow:

```bash
if ! n8n health check; then
  case $? in
    69) echo "n8n not reachable" ;;
    77) echo "Auth failed" ;;
  esac
fi
```

---

## See Also

- [Scripting Guide](../guides/scripting.md) - Using exit codes in scripts
- [Architecture](./architecture.md) - Error handling flow
- [Adding Commands](./adding-commands.md) - Creating new error types
