# Code Structure

Detailed breakdown of the codebase organization and file purposes.

## Directory Tree

```
src/
├── main.rs                 # Entry point and command handlers
├── lib.rs                  # Public module exports
├── error.rs                # Error types and exit codes
│
├── cli/                    # Command-line interface definitions
│   ├── mod.rs              # Module exports
│   ├── app.rs              # Main CLI structure
│   ├── workflows.rs        # Workflow commands
│   ├── executions.rs       # Execution commands
│   ├── credentials.rs      # Credential commands
│   ├── tags.rs             # Tag commands
│   └── health.rs           # Health check commands
│
├── client/                 # HTTP client and API endpoints
│   ├── mod.rs              # Module exports
│   ├── api.rs              # Core HTTP client (N8nClient)
│   ├── pagination.rs       # Pagination types
│   └── endpoints/          # Domain-specific endpoints
│       ├── mod.rs          # Module exports
│       ├── workflows.rs    # Workflow API methods
│       ├── executions.rs   # Execution API methods
│       ├── credentials.rs  # Credential API methods
│       ├── tags.rs         # Tag API methods
│       └── health.rs       # Health API methods
│
├── config/                 # Configuration loading
│   ├── mod.rs              # Module exports
│   └── loader.rs           # Config file parsing and merging
│
├── models/                 # Data structures
│   ├── mod.rs              # Module exports
│   ├── workflow.rs         # Workflow types
│   ├── node.rs             # Node types
│   ├── connection.rs       # Connection types
│   ├── execution.rs        # Execution types
│   ├── credential.rs       # Credential types
│   ├── tag.rs              # Tag types
│   └── common.rs           # Shared utilities
│
├── output/                 # Output formatting
│   ├── mod.rs              # Module exports and Outputable trait
│   ├── format.rs           # Format selection logic
│   ├── table.rs            # Table output
│   └── json.rs             # JSON output
│
├── validation/             # Workflow validation
│   ├── mod.rs              # Module exports
│   └── workflow.rs         # Validation rules
│
├── diff/                   # Workflow comparison
│   ├── mod.rs              # Module exports
│   └── workflow_diff.rs    # Diff logic
│
└── editor/                 # External editor support
    ├── mod.rs              # Module exports
    └── external.rs         # Editor integration
```

## Core Files

### `src/main.rs`

Entry point and command handler orchestration.

**Responsibilities:**
- Initialize tokio runtime
- Parse CLI arguments
- Load and validate configuration
- Create API client
- Dispatch to command handlers
- Handle errors and exit codes

**Key functions:**
- `main()` - Entry point
- `run()` - Async main logic
- `handle_workflows()` - Workflow command handler
- `handle_executions()` - Execution command handler
- `handle_credentials()` - Credential command handler
- `handle_tags()` - Tag command handler
- `handle_health()` - Health command handler

### `src/lib.rs`

Public API exports for the crate.

```rust
pub mod cli;
pub mod client;
pub mod config;
pub mod diff;
pub mod editor;
pub mod error;
pub mod models;
pub mod output;
pub mod validation;

pub use cli::{Cli, Commands};
pub use client::N8nClient;
pub use config::{load_config, validate_config, CliOverrides, Config};
pub use error::{N8nError, Result};
pub use output::{OutputFormat, Outputable, print_output, print_single};
```

### `src/error.rs`

Unified error handling.

**Types:**
- `N8nError` - Enum of all error variants
- `ApiErrorResponse` - n8n API error structure
- `Result<T>` - Type alias for `Result<T, N8nError>`

**Key method:**
- `N8nError::exit_code()` - Maps errors to Unix exit codes

## CLI Module (`src/cli/`)

### `app.rs`

Main CLI structure using clap derive.

```rust
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(global = true, short = 'p', long)]
    pub profile: Option<String>,

    #[arg(global = true, long)]
    pub url: Option<String>,

    // ... more global args
}

#[derive(Subcommand)]
pub enum Commands {
    Workflows(WorkflowsCommand),
    Executions(ExecutionsCommand),
    // ...
}
```

### Command Files

Each command has its own file with:
- Main command struct with `#[derive(Args)]`
- Action enum with `#[derive(Subcommand)]`
- Options for each action

Example (`workflows.rs`):
```rust
#[derive(Args)]
pub struct WorkflowsCommand {
    #[command(subcommand)]
    pub action: WorkflowsAction,
}

#[derive(Subcommand)]
pub enum WorkflowsAction {
    List { /* options */ },
    Get { id: String },
    Create { file: PathBuf, /* options */ },
    // ...
}
```

## Client Module (`src/client/`)

### `api.rs`

Core HTTP client implementation.

```rust
pub struct N8nClient {
    client: reqwest::Client,
    base_url: String,
}

impl N8nClient {
    pub fn new(base_url: &str, api_key: &str) -> Result<Self>;

    // HTTP methods
    pub async fn get<T>(&self, path: &str) -> Result<T>;
    pub async fn get_with_query<T, Q>(&self, path: &str, query: &Q) -> Result<T>;
    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>;
    pub async fn post_empty<T>(&self, path: &str) -> Result<T>;
    pub async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>;
    pub async fn delete(&self, path: &str) -> Result<()>;
}
```

### `pagination.rs`

Pagination support types.

```rust
#[derive(Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
}
```

### Endpoint Files

Each domain has methods on `N8nClient`:

```rust
// In endpoints/workflows.rs
impl N8nClient {
    pub async fn list_workflows(&self, params: &WorkflowListParams) -> Result<Vec<Workflow>>;
    pub async fn get_workflow(&self, id: &str) -> Result<WorkflowDetail>;
    pub async fn create_workflow(&self, workflow: &WorkflowDefinition) -> Result<WorkflowDetail>;
    // ...
}
```

## Config Module (`src/config/`)

### `loader.rs`

Configuration loading and merging.

**Types:**
- `Config` - Runtime configuration
- `ConfigFile` - TOML file structure
- `Profile` - Named profile settings
- `CliOverrides` - CLI argument overrides

**Functions:**
- `load_config(overrides: CliOverrides) -> Result<Config>`
- `validate_config(config: &Config) -> Result<()>`

## Models Module (`src/models/`)

### Data Structures

Each file defines related types:

**`workflow.rs`:**
- `Workflow` - Summary (for listings)
- `WorkflowDetail` - Full workflow with nodes/connections
- `TypedWorkflow` - Parsed workflow with typed nodes
- `WorkflowDefinition` - For create/update operations

**`node.rs`:**
- `Node` - Node definition
- `Position` - Custom serialization for `[x, y]`

**`connection.rs`:**
- `Connection` - Flattened connection format
- `ConnectionsMap` - n8n's nested format
- Conversion methods between formats

**`execution.rs`:**
- `Execution` - Execution summary
- `ExecutionDetail` - Full execution with data

**`credential.rs`:**
- `Credential` - Credential metadata
- `CredentialCreate` - Create request

**`tag.rs`:**
- `Tag` - Tag definition
- `TagCreate`, `TagUpdate` - Request types

### `common.rs`

Shared utilities:
- `read_json_file()` - Read and parse JSON
- `read_stdin()` - Read from stdin

## Output Module (`src/output/`)

### `mod.rs` / `format.rs`

```rust
pub trait Outputable {
    fn headers() -> Vec<&'static str>;
    fn row(&self) -> Vec<String>;
}

pub enum OutputFormat {
    Table,
    Json,
    JsonPretty,
}

pub fn print_output<T: Outputable + Serialize>(
    items: &[T],
    format: OutputFormat,
) -> Result<()>;

pub fn print_single<T: Serialize>(
    item: &T,
    format: OutputFormat,
) -> Result<()>;
```

### `table.rs`

Table formatting using comfy-table.

### `json.rs`

JSON serialization with optional pretty printing.

## Validation Module (`src/validation/`)

### `workflow.rs`

```rust
pub struct ValidationResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn validate_workflow(workflow: &TypedWorkflow) -> ValidationResult;
```

## Diff Module (`src/diff/`)

### `workflow_diff.rs`

```rust
pub struct WorkflowDiff {
    pub name_changed: Option<(String, String)>,
    pub active_changed: Option<(bool, bool)>,
    pub nodes_added: Vec<String>,
    pub nodes_removed: Vec<String>,
    pub nodes_modified: Vec<NodeDiff>,
    pub connections_added: Vec<Connection>,
    pub connections_removed: Vec<Connection>,
}

impl WorkflowDiff {
    pub fn compare(a: &TypedWorkflow, b: &TypedWorkflow) -> Self;
    pub fn is_empty(&self) -> bool;
    pub fn print_summary(&self);
    pub fn print_full(&self);
}
```

## Editor Module (`src/editor/`)

### `external.rs`

```rust
pub fn edit_workflow(workflow: &WorkflowDetail, editor: Option<&str>) -> Result<Value>;
```

Opens workflow in external editor, waits for changes, returns parsed JSON.

## Key Types Summary

| Type | Location | Purpose |
|------|----------|---------|
| `Cli` | `cli/app.rs` | Root CLI structure |
| `Commands` | `cli/app.rs` | Top-level command enum |
| `N8nClient` | `client/api.rs` | HTTP client |
| `Config` | `config/loader.rs` | Runtime configuration |
| `N8nError` | `error.rs` | Error enum |
| `Workflow` | `models/workflow.rs` | Workflow summary |
| `WorkflowDetail` | `models/workflow.rs` | Full workflow |
| `Node` | `models/node.rs` | Node definition |
| `Connection` | `models/connection.rs` | Connection definition |
| `Outputable` | `output/mod.rs` | Table formatting trait |
| `ValidationResult` | `validation/workflow.rs` | Validation output |
| `WorkflowDiff` | `diff/workflow_diff.rs` | Comparison result |

---

## See Also

- [Architecture](./architecture.md) - High-level design
- [Adding Commands](./adding-commands.md) - Extend the CLI
- [Models](./models.md) - Data structure details
