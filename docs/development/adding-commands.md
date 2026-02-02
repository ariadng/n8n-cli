# Adding Commands

Step-by-step guide for adding new commands to the n8n CLI.

## Overview

Adding a new command involves these steps:

1. Define CLI structure (clap)
2. Add command to main enum
3. Implement handler in main.rs
4. Add API endpoint methods
5. Create data models
6. Implement output formatting

## Example: Adding a "Variables" Command

Let's walk through adding a hypothetical `variables` command for managing n8n variables.

### Step 1: Define CLI Structure

Create `src/cli/variables.rs`:

```rust
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct VariablesCommand {
    #[command(subcommand)]
    pub action: VariablesAction,
}

#[derive(Subcommand)]
pub enum VariablesAction {
    /// List all variables
    List,

    /// Get a variable by key
    Get {
        /// Variable key
        key: String,
    },

    /// Set a variable
    Set {
        /// Variable key
        key: String,

        /// Variable value
        value: String,

        /// Variable type (string, number, boolean)
        #[arg(long, short = 't', default_value = "string")]
        r#type: String,
    },

    /// Delete a variable
    Delete {
        /// Variable key
        key: String,

        /// Skip confirmation
        #[arg(long, short = 'f')]
        force: bool,
    },
}
```

### Step 2: Export from CLI Module

Update `src/cli/mod.rs`:

```rust
mod app;
mod credentials;
mod executions;
mod health;
mod tags;
mod variables;  // Add this
mod workflows;

pub use app::{Cli, Commands};
pub use credentials::{CredentialsAction, CredentialsCommand};
pub use executions::{ExecutionsAction, ExecutionsCommand};
pub use health::{HealthAction, HealthCommand};
pub use tags::{TagsAction, TagsCommand};
pub use variables::{VariablesAction, VariablesCommand};  // Add this
pub use workflows::{WorkflowsAction, WorkflowsCommand, /* ... */};
```

### Step 3: Add to Commands Enum

Update `src/cli/app.rs`:

```rust
use super::variables::VariablesCommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Manage workflows
    #[command(alias = "wf")]
    Workflows(WorkflowsCommand),

    /// Manage executions
    #[command(alias = "exec")]
    Executions(ExecutionsCommand),

    // ... existing commands ...

    /// Manage variables
    #[command(alias = "var")]
    Variables(VariablesCommand),
}
```

### Step 4: Create Data Models

Create `src/models/variable.rs`:

```rust
use serde::{Deserialize, Serialize};
use crate::output::Outputable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub id: String,
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub variable_type: String,
}

impl Outputable for Variable {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "Key", "Type", "Value"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.key.clone(),
            self.variable_type.clone(),
            self.value.clone(),
        ]
    }
}

#[derive(Debug, Serialize)]
pub struct VariableCreate {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub variable_type: String,
}
```

Update `src/models/mod.rs`:

```rust
mod variable;
pub use variable::{Variable, VariableCreate};
```

### Step 5: Add API Endpoints

Create `src/client/endpoints/variables.rs`:

```rust
use crate::error::Result;
use crate::models::{Variable, VariableCreate};
use super::super::N8nClient;

impl N8nClient {
    pub async fn list_variables(&self) -> Result<Vec<Variable>> {
        self.get("/variables").await
    }

    pub async fn get_variable(&self, key: &str) -> Result<Variable> {
        self.get(&format!("/variables/{}", key)).await
    }

    pub async fn set_variable(&self, variable: &VariableCreate) -> Result<Variable> {
        // Check if exists first, then PUT or POST
        match self.get_variable(&variable.key).await {
            Ok(_) => self.put(&format!("/variables/{}", variable.key), variable).await,
            Err(_) => self.post("/variables", variable).await,
        }
    }

    pub async fn delete_variable(&self, key: &str) -> Result<()> {
        self.delete(&format!("/variables/{}", key)).await
    }
}
```

Update `src/client/endpoints/mod.rs`:

```rust
mod credentials;
mod executions;
mod health;
mod tags;
mod variables;  // Add this
mod workflows;
```

### Step 6: Implement Command Handler

Add to `src/main.rs`:

```rust
use n8n_cli::cli::{VariablesAction, VariablesCommand};
use n8n_cli::models::VariableCreate;

// In the match block for Commands:
Commands::Variables(cmd) => {
    handle_variables(&client, cmd.action, &config).await?;
}

// Add handler function:
async fn handle_variables(
    client: &N8nClient,
    action: VariablesAction,
    config: &Config,
) -> Result<()> {
    match action {
        VariablesAction::List => {
            let variables = client.list_variables().await?;
            print_output(&variables, config.output_format)?;
        }

        VariablesAction::Get { key } => {
            let variable = client.get_variable(&key).await?;
            print_single(&variable, config.output_format)?;
        }

        VariablesAction::Set { key, value, r#type } => {
            let variable = VariableCreate {
                key: key.clone(),
                value,
                variable_type: r#type,
            };
            let result = client.set_variable(&variable).await?;
            if !config.quiet {
                eprintln!("Variable '{}' set successfully", key);
            }
            print_single(&result, config.output_format)?;
        }

        VariablesAction::Delete { key, force } => {
            if !force {
                eprint!("Delete variable '{}'? [y/N] ", key);
                // Read confirmation...
            }
            client.delete_variable(&key).await?;
            if !config.quiet {
                eprintln!("Variable '{}' deleted", key);
            }
        }
    }

    Ok(())
}
```

### Step 7: Add Error Variants (if needed)

Update `src/error.rs`:

```rust
#[derive(Debug, Error)]
pub enum N8nError {
    // ... existing variants ...

    #[error("Variable '{0}' not found")]
    VariableNotFound(String),
}

impl N8nError {
    pub fn exit_code(&self) -> i32 {
        match self {
            // ... existing matches ...
            Self::VariableNotFound(_) => 69,  // EX_UNAVAILABLE
        }
    }
}
```

## Checklist

When adding a new command:

- [ ] CLI structure in `src/cli/<domain>.rs`
- [ ] Export in `src/cli/mod.rs`
- [ ] Add to `Commands` enum in `src/cli/app.rs`
- [ ] Data models in `src/models/<domain>.rs`
- [ ] Export in `src/models/mod.rs`
- [ ] Implement `Outputable` for table output
- [ ] API endpoints in `src/client/endpoints/<domain>.rs`
- [ ] Export in `src/client/endpoints/mod.rs`
- [ ] Handler in `src/main.rs`
- [ ] Error variants if needed
- [ ] Update documentation

## Best Practices

### CLI Design

```rust
// Use descriptive help text
/// List all variables with optional filtering
List {
    /// Filter by key prefix
    #[arg(long, short = 'p')]
    prefix: Option<String>,
}

// Provide short aliases
#[arg(long, short = 'f')]
force: bool,

// Use sensible defaults
#[arg(long, default_value = "100")]
limit: u32,
```

### Error Handling

```rust
// Use specific error types
let variable = client.get_variable(&key).await
    .map_err(|_| N8nError::VariableNotFound(key.clone()))?;

// Add context where helpful
let content = std::fs::read_to_string(&path)
    .map_err(|e| N8nError::FileRead {
        path: path.display().to_string(),
        source: e,
    })?;
```

### Output

```rust
// Use quiet mode for status messages
if !config.quiet {
    eprintln!("Created variable: {}", key);
}

// Always use print_output for data
print_output(&variables, config.output_format)?;
```

### Models

```rust
// Use serde rename for API compatibility
#[serde(rename = "createdAt")]
pub created_at: String,

// Skip None values in output
#[serde(skip_serializing_if = "Option::is_none")]
pub description: Option<String>,
```

## Testing Manually

```bash
# Build
cargo build

# Test the new command
./target/debug/n8n variables list
./target/debug/n8n var get MY_VAR
./target/debug/n8n var set MY_VAR "my value" -t string
./target/debug/n8n var delete MY_VAR -f

# Test output formats
./target/debug/n8n var list -o json
./target/debug/n8n var list -o json-pretty

# Test help
./target/debug/n8n variables --help
./target/debug/n8n variables list --help
```

---

## See Also

- [Code Structure](./code-structure.md) - File organization
- [API Client](./api-client.md) - HTTP client details
- [Models](./models.md) - Data structure patterns
- [Error Handling](./error-handling.md) - Error conventions
