# Contributing to Rust CLI Utilities

Thank you for your interest in contributing to this project. This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful, constructive, and professional in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/rustcli.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes
6. Submit a pull request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git
- A code editor (VS Code with rust-analyzer recommended)

See `SETUP.md` for detailed setup instructions.

### Building

```bash
cargo build --all-features
```

### Running Tests

```bash
cargo test
```

## Contribution Guidelines

### Code Quality Standards

All contributions must meet these standards:

#### 1. Formatting

Code must be formatted with rustfmt:

```bash
cargo fmt --all
```

#### 2. Linting

Code must pass clippy without warnings:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

#### 3. Testing

- All new features must include tests
- All tests must pass: `cargo test`
- Aim for high test coverage
- Include both unit tests and integration tests where appropriate

#### 4. Documentation

- Public APIs must have doc comments
- Include usage examples in doc comments
- Update README files if adding new features
- Add inline comments for complex logic

#### 5. Error Handling

- No `.unwrap()` or `.expect()` in production code
- Use `?` operator for error propagation
- Provide meaningful error messages
- Use `anyhow::Context` for error context

### Commit Message Format

Use clear, descriptive commit messages:

```
<type>: <short summary>

<detailed description if needed>

<footer: references to issues, breaking changes, etc.>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Build process or tooling changes

Examples:

```
feat: add -r flag to cat for recursive directory processing

Implements recursive directory traversal for cat utility.
Files are processed in alphabetical order.

Closes #42
```

```
fix: handle permission errors gracefully in ls

Previously, ls would crash when encountering files without
read permissions. Now it shows a warning and continues.
```

## What to Contribute

### Good First Issues

Look for issues tagged with "good first issue" for newcomer-friendly tasks.

### Feature Requests

Before implementing a new feature:

1. Check if an issue exists
2. If not, create an issue describing the feature
3. Wait for maintainer feedback
4. Implement after approval

### Bug Fixes

1. Create an issue describing the bug (if one doesn't exist)
2. Reference the issue in your pull request
3. Include a test that reproduces the bug
4. Ensure the test passes with your fix

### Areas for Contribution

**High Priority:**
- Implementing find utility
- Implementing grep utility
- Adding colorized output
- Performance optimizations
- Cross-platform testing

**Medium Priority:**
- Shell completion scripts
- Man page generation
- Additional flags for existing utilities
- Improved error messages

**Low Priority:**
- Code refactoring
- Documentation improvements
- Additional examples

## Testing Guidelines

### Unit Tests

Place unit tests in the same file as the code being tested:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Test implementation
    }
}
```

### Integration Tests

Place integration tests in the `tests/` directory:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_behavior() {
    let mut cmd = Command::cargo_bin("utility-name").unwrap();
    cmd.arg("--flag");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("expected output"));
}
```

### Test Coverage

Aim to test:
- Happy paths
- Error conditions
- Edge cases (empty input, very large files, etc.)
- Different platforms when relevant

## Pull Request Process

1. **Before submitting:**
   - Run `cargo fmt --all`
   - Run `cargo clippy --all-targets --all-features -- -D warnings`
   - Run `cargo test`
   - Update documentation
   - Add tests for new functionality

2. **PR Description:**
   - Clearly describe what the PR does
   - Reference related issues
   - Include screenshots for UI changes
   - List any breaking changes

3. **Review Process:**
   - Maintainers will review your PR
   - Address any requested changes
   - Keep the discussion constructive
   - Be patient - reviews may take time

4. **After Merge:**
   - Delete your feature branch
   - Pull the latest main branch
   - Your contribution will be in the next release

## Code Style Guidelines

### Naming Conventions

- Functions and variables: `snake_case`
- Types and traits: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

### File Organization

```rust
// Imports - grouped and sorted
use std::io;
use anyhow::Result;

// Constants
const MAX_SIZE: usize = 1024;

// Type definitions
struct MyStruct { ... }

// Implementations
impl MyStruct { ... }

// Public functions
pub fn public_function() { ... }

// Private functions
fn private_function() { ... }

// Tests
#[cfg(test)]
mod tests { ... }
```

### Error Handling Pattern

```rust
use anyhow::{Context, Result};

fn process_file(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path))?;
    
    Ok(content)
}
```

### Documentation Comments

```rust
/// Processes a file and returns its contents.
///
/// # Arguments
///
/// * `path` - Path to the file to process
///
/// # Returns
///
/// The file contents as a string
///
/// # Errors
///
/// Returns an error if the file cannot be read or is not valid UTF-8.
///
/// # Examples
///
/// ```
/// let content = process_file("example.txt")?;
/// ```
pub fn process_file(path: &str) -> Result<String> {
    // Implementation
}
```

## Feature Flags

When adding optional functionality:

1. Add the feature to `Cargo.toml`
2. Gate the code with `#[cfg(feature = "feature-name")]`
3. Document the feature in README
4. Ensure tests pass with and without the feature

Example:

```toml
[features]
default = []
color = ["dep:colored"]

[dependencies]
colored = { version = "2.1", optional = true }
```

## Platform-Specific Code

When writing platform-specific code:

```rust
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(windows)]
fn platform_specific() {
    // Windows implementation
}

#[cfg(not(windows))]
fn platform_specific() {
    // Unix/macOS implementation
}
```

## Questions?

If you have questions:

1. Check existing documentation
2. Search closed issues
3. Open a new issue with the "question" label

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.

