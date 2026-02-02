# Contributing

Guidelines for contributing to the n8n CLI project.

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Git
- Access to an n8n instance for testing

### Clone and Build

```bash
git clone https://github.com/your-org/n8n-cli.git
cd n8n-cli

# Build debug version
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run clippy (linter)
cargo clippy

# Format code
cargo fmt
```

### Development Configuration

Create a local config for testing:

```bash
mkdir -p ~/.config/n8n-cli

cat > ~/.config/n8n-cli/config.toml << 'EOF'
default_profile = "dev"
output_format = "table"

[profiles.dev]
base_url = "http://localhost:5678"
api_key_env = "N8N_DEV_API_KEY"
EOF

export N8N_DEV_API_KEY="your-test-api-key"
```

### Running During Development

```bash
# Run debug build
cargo run -- workflows list

# Run with arguments
cargo run -- -o json workflows get wf_123

# With environment override
N8N_BASE_URL=http://localhost:5678 cargo run -- health check
```

## Code Style

### General Principles

1. **KISS** - Keep implementations simple and straightforward
2. **DRY** - Extract reusable components, avoid duplication
3. **Self-documenting** - Use descriptive names over comments

### Rust Conventions

```rust
// Use snake_case for functions and variables
fn get_workflow_by_id(workflow_id: &str) -> Result<Workflow>

// Use CamelCase for types
struct WorkflowDetail { ... }

// Use SCREAMING_SNAKE_CASE for constants
const DEFAULT_TIMEOUT: u64 = 30;

// Prefer explicit types for public APIs
pub fn list_workflows(params: &WorkflowListParams) -> Result<Vec<Workflow>>
```

### Error Handling

```rust
// Always use the project's Result type
use crate::error::Result;

// Propagate errors with ?
let data = client.get(url).await?;

// Add context when helpful
let content = fs::read_to_string(&path)
    .map_err(|e| N8nError::FileRead {
        path: path.to_string(),
        source: e,
    })?;
```

### Documentation

```rust
/// Brief description of the function.
///
/// More detailed explanation if needed.
///
/// # Arguments
///
/// * `id` - The workflow ID
///
/// # Returns
///
/// The workflow details on success
///
/// # Errors
///
/// Returns `WorkflowNotFound` if the workflow doesn't exist
pub async fn get_workflow(&self, id: &str) -> Result<WorkflowDetail> {
    // ...
}
```

### Formatting

Always run before committing:

```bash
cargo fmt
```

### Linting

Fix all clippy warnings:

```bash
cargo clippy -- -D warnings
```

## Project Structure

When adding new features, follow the existing patterns:

```
src/
├── cli/           # CLI argument definitions (clap)
├── client/        # HTTP client and API endpoints
│   └── endpoints/ # Domain-specific API methods
├── config/        # Configuration loading
├── models/        # Data structures
├── output/        # Output formatting
├── validation/    # Validation logic
├── diff/          # Comparison logic
├── editor/        # External editor integration
├── error.rs       # Error types
├── lib.rs         # Public exports
└── main.rs        # Entry point and handlers
```

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 2. Make Changes

Follow the patterns in existing code:

- CLI definitions in `src/cli/`
- API methods in `src/client/endpoints/`
- Models in `src/models/`
- Handlers in `src/main.rs`

### 3. Test Locally

```bash
# Build
cargo build

# Test against local n8n
./target/debug/n8n workflows list

# Run unit tests
cargo test

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy
```

### 4. Write Commit Message

```
type: Brief description (50 chars or less)

More detailed explanation if needed. Wrap at 72 characters.

- Bullet points are fine
- Keep them concise

Closes #123
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, no code change
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `test`: Adding tests
- `chore`: Maintenance tasks

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Create a pull request with:
- Clear description of changes
- Link to related issues
- Test instructions if applicable

## Pull Request Guidelines

### Before Submitting

- [ ] Code compiles without warnings (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated if needed
- [ ] Commit messages follow convention

### PR Description Template

```markdown
## Summary
Brief description of changes.

## Changes
- Added X
- Fixed Y
- Updated Z

## Testing
How to test these changes:
1. Step one
2. Step two

## Related Issues
Closes #123
```

### Review Process

1. Automated checks must pass
2. At least one maintainer review
3. Address all feedback
4. Squash commits if requested

## Adding Features

### New Command Checklist

- [ ] CLI struct in `src/cli/<domain>.rs`
- [ ] Export in `src/cli/mod.rs`
- [ ] Add to `Commands` enum
- [ ] Data models in `src/models/<domain>.rs`
- [ ] Export in `src/models/mod.rs`
- [ ] Implement `Outputable` trait
- [ ] API endpoint in `src/client/endpoints/<domain>.rs`
- [ ] Export in `src/client/endpoints/mod.rs`
- [ ] Handler in `src/main.rs`
- [ ] Error variants if needed
- [ ] Update documentation

See [Adding Commands](./adding-commands.md) for detailed walkthrough.

## Testing

### Manual Testing

Test against a real n8n instance:

```bash
# Basic operations
n8n wf list
n8n wf get <id>
n8n wf create test-workflow.json

# Different output formats
n8n wf list -o json
n8n wf list -o json-pretty

# Error cases
n8n wf get nonexistent-id  # Should return error
n8n --url invalid health check  # Should fail gracefully
```

### Unit Tests

Add tests in the same file or `tests/` directory:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_serialization() {
        let pos = Position { x: 100, y: 200 };
        let json = serde_json::to_string(&pos).unwrap();
        assert_eq!(json, "[100,200]");
    }

    #[test]
    fn test_connection_conversion() {
        // Test connection map conversion
    }
}
```

## Documentation

### Code Documentation

- Public items should have doc comments
- Use `///` for item documentation
- Use `//` for implementation notes

### User Documentation

Update `docs/` when:
- Adding new commands
- Changing command behavior
- Adding new features

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create git tag: `git tag v0.1.0`
4. Push tag: `git push origin v0.1.0`
5. CI builds and publishes release

## Getting Help

- Open an issue for bugs or feature requests
- Start a discussion for questions
- Tag maintainers for urgent issues

## License

Contributions are licensed under the same terms as the project (see LICENSE file).

---

Thank you for contributing!
