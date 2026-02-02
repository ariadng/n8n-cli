# Architecture

High-level overview of the n8n CLI architecture and design principles.

## Design Principles

The project follows these core principles:

1. **KISS (Keep It Simple, Stupid)** - Favor simple, straightforward solutions
2. **DRY (Don't Repeat Yourself)** - Extract reusable components
3. **Self-documenting code** - Use descriptive names and types over comments

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         CLI Layer                                │
│  ┌─────────┐ ┌──────────┐ ┌───────────┐ ┌──────┐ ┌────────┐    │
│  │workflows│ │executions│ │credentials│ │ tags │ │ health │    │
│  └────┬────┘ └────┬─────┘ └─────┬─────┘ └──┬───┘ └───┬────┘    │
└───────┼───────────┼─────────────┼──────────┼─────────┼──────────┘
        │           │             │          │         │
        ▼           ▼             ▼          ▼         ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Handler Layer (main.rs)                     │
│    Parse args → Load config → Create client → Dispatch → Output │
└─────────────────────────────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Client Layer                              │
│  ┌───────────────────┐  ┌─────────────────────────────────────┐ │
│  │    N8nClient      │  │           Endpoints                 │ │
│  │  ─────────────    │  │  ┌──────────┐ ┌──────────────────┐  │ │
│  │  - base_url       │──│  │workflows │ │   executions     │  │ │
│  │  - http_client    │  │  └──────────┘ └──────────────────┘  │ │
│  │  - api_key        │  │  ┌──────────┐ ┌───────┐ ┌────────┐  │ │
│  └───────────────────┘  │  │credentials│ │ tags │ │ health │  │ │
│                         │  └──────────┘ └───────┘ └────────┘  │ │
│                         └─────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Model Layer                               │
│  ┌──────────┐ ┌──────┐ ┌──────────┐ ┌─────────┐ ┌────────────┐ │
│  │ Workflow │ │ Node │ │Connection│ │Execution│ │ Credential │ │
│  └──────────┘ └──────┘ └──────────┘ └─────────┘ └────────────┘ │
└─────────────────────────────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Support Modules                             │
│  ┌────────┐ ┌──────────┐ ┌───────┐ ┌────────────┐ ┌──────────┐ │
│  │ Config │ │  Output  │ │ Error │ │ Validation │ │   Diff   │ │
│  └────────┘ └──────────┘ └───────┘ └────────────┘ └──────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Layer Responsibilities

### CLI Layer (`src/cli/`)

Defines the command-line interface structure using clap derive macros.

- **Responsibility**: Argument parsing, help generation, validation
- **No business logic**: Only defines structure and constraints
- **Files**: `app.rs`, `workflows.rs`, `executions.rs`, `credentials.rs`, `tags.rs`, `health.rs`

### Handler Layer (`src/main.rs`)

Orchestrates command execution and error handling.

- **Flow control**: Parse → Configure → Execute → Output
- **Error translation**: Convert errors to exit codes
- **Output formatting**: Apply user's output format preference

### Client Layer (`src/client/`)

HTTP client for n8n API communication.

- **N8nClient**: Core HTTP client with authentication
- **Endpoints**: Domain-specific API methods (CRUD operations)
- **Pagination**: Transparent handling of paginated responses

### Model Layer (`src/models/`)

Data structures matching n8n API responses.

- **Serialization**: serde for JSON conversion
- **Outputable trait**: For table/JSON formatting
- **Type safety**: Rust types prevent invalid states

### Support Modules

- **Config** (`src/config/`): Hierarchical configuration loading
- **Output** (`src/output/`): Table and JSON formatters
- **Error** (`src/error.rs`): Unified error types with exit codes
- **Validation** (`src/validation/`): Workflow structure validation
- **Diff** (`src/diff/`): Workflow comparison logic
- **Editor** (`src/editor/`): External editor integration

## Request Flow

```
User Input
    │
    ▼
┌───────────────────┐
│  CLI Parsing      │  clap parses arguments
│  (Cli::parse())   │
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│  Config Loading   │  Load file → env → CLI overrides
│  (load_config())  │
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│  Config Validation│  Check required values
│  (validate_config)│
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│  Client Creation  │  N8nClient::new(url, api_key)
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│  Command Dispatch │  Match on Commands enum
│  (handle_*)       │
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│  API Call         │  client.get/post/put/delete
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│  Output Format    │  print_output() / print_single()
└─────────┬─────────┘
          │
          ▼
    Exit Code
```

## Error Handling Strategy

Errors are handled at the boundary (main.rs):

```rust
fn main() {
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(run());

    match result {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(e.exit_code());
        }
    }
}
```

All functions return `Result<T, N8nError>`:
- Operations propagate errors with `?`
- Context is added where helpful
- Final error is formatted for users

## Configuration Resolution

Layered configuration with clear precedence:

```
CLI Flags (highest)
    ↓
Environment Variables
    ↓
Config File Profile
    ↓
Config File Defaults
    ↓
Built-in Defaults (lowest)
```

## Output Architecture

The `Outputable` trait enables consistent formatting:

```rust
pub trait Outputable {
    fn headers() -> Vec<&'static str>;
    fn row(&self) -> Vec<String>;
}
```

Models implement this trait, and `print_output()` handles the format selection:

```rust
match format {
    OutputFormat::Table => print_table(items),
    OutputFormat::Json => print_json(items),
    OutputFormat::JsonPretty => print_json_pretty(items),
}
```

## Async Runtime

The CLI uses Tokio for async HTTP operations:

```rust
#[tokio::main]
async fn main() {
    // All async operations use tokio runtime
}
```

Benefits:
- Non-blocking HTTP requests
- Potential for concurrent operations
- Standard async/await syntax

## Extensibility Points

### Adding a New Command

1. Add CLI struct in `src/cli/`
2. Add handler in `src/main.rs`
3. Add endpoint method in `src/client/endpoints/`
4. Add models in `src/models/`

### Adding a New Output Format

1. Add variant to `OutputFormat` enum
2. Implement formatting in `src/output/`
3. Update `print_output()` function

### Adding New Validation Rules

1. Add check in `src/validation/workflow.rs`
2. Return error or warning appropriately

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `tokio` | Async runtime |
| `reqwest` | HTTP client |
| `serde` | Serialization |
| `thiserror` | Error derive macros |
| `comfy-table` | Table formatting |
| `similar` | Diff generation |

## Performance Considerations

- **Binary size**: LTO and stripping enabled in release
- **Startup time**: Minimal initialization, lazy client creation
- **Memory**: Stream large responses when possible
- **Network**: Connection pooling via reqwest

---

## See Also

- [Code Structure](./code-structure.md) - Detailed module breakdown
- [Adding Commands](./adding-commands.md) - Step-by-step guide
- [API Client](./api-client.md) - HTTP client details
