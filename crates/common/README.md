# Common

Shared utilities library for the Rust CLI utilities workspace.

## Purpose

This crate provides common functionality used across multiple CLI tools, including:

- Error type definitions
- I/O helper functions
- Color output support (optional)
- Testing utilities

## Modules

### error

Defines common error types using `thiserror`:

```rust
use common::error::{CommonError, Result};

fn example() -> Result<String> {
    // Error handling with proper types
    Ok("success".to_string())
}
```

Error variants:
- `Io`: I/O errors
- `Utf8`: UTF-8 conversion errors
- `InvalidArgument`: Invalid arguments
- `FileNotFound`: File not found
- `PermissionDenied`: Permission denied

### io

Provides buffered I/O helpers:

```rust
use common::io;

// Open file or stdin
let reader = io::open_input("file.txt")?;

// Buffered stdout
let mut stdout = io::stdout_writer();

// Read all bytes
let bytes = io::read_all_bytes(reader)?;
```

Functions:
- `open_input(path)`: Returns buffered reader for file or stdin
- `buffered_reader(path)`: Creates buffered file reader
- `stdout_writer()`: Buffered stdout writer
- `stderr_writer()`: Buffered stderr writer
- `read_all_bytes(reader)`: Read all bytes from reader
- `write_and_flush(writer, data)`: Write and flush data
- `count_lines(reader)`: Count lines in reader

### color (optional feature)

Provides color output support using the `colored` crate:

```rust
#[cfg(feature = "color")]
use common::color::{ColorConfig, schemes};

// Configure colors
let config = ColorConfig::new(true);

// Use color schemes
let colored_dir = schemes::directory("mydir");
let colored_error = schemes::error("error message");
```

Color schemes:
- `directory()`: Blue bold for directories
- `executable()`: Green bold for executables
- `symlink()`: Cyan for symlinks
- `error()`: Red bold for errors
- `warning()`: Yellow for warnings
- `success()`: Green for success messages

### testing

Testing utilities for creating fixtures:

```rust
#[cfg(test)]
use common::testing::{TestFixture, create_test_file};

let fixture = TestFixture::new(temp_dir);
fixture.create_file("test.txt", "content")?;
fixture.create_dir("subdir")?;
```

Functions:
- `create_test_file(path, content)`: Create test file
- `create_test_dir(path)`: Create test directory
- `TestFixture`: Helper for managing test fixtures

## Features

### Default Features

By default, the crate includes only core functionality (error types and I/O helpers).

### Optional Features

**color**: Enables terminal color output support
```bash
cargo build --features color
```

## Usage in Workspace

Add to your binary crate's `Cargo.toml`:

```toml
[dependencies]
common = { path = "../common" }

# Or with features
common = { path = "../common", features = ["color"] }
```

## Design Principles

**Library Error Handling:**
- Uses `thiserror` for structured errors
- No `anyhow` dependency (libraries should use transparent errors)
- Proper error context and source chaining

**Zero Unwrap Policy:**
- No `.unwrap()` or `.expect()` calls
- All errors properly propagated

**Feature Gating:**
- Optional dependencies behind feature flags
- Minimal default surface area

**Cross-Platform:**
- Standard library abstractions
- No platform-specific code in core modules

## Testing

```bash
# Run tests
cargo test

# Run tests with all features
cargo test --all-features
```

## License

Licensed under the Apache License, Version 2.0.

