# Setup Guide

This guide will help you get started with the Rust CLI Utilities workspace.

## Prerequisites

### Install Rust

You need to install Rust and Cargo to build this project.

#### Windows

1. Download and run rustup-init.exe from https://rustup.rs/
2. Follow the on-screen instructions
3. Restart your terminal after installation
4. Verify installation:
   ```powershell
   cargo --version
   rustc --version
   ```

#### Linux/macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cargo --version
rustc --version
```

### Verify Installation

After installing Rust, verify that it works:

```bash
cargo --version
# Should output something like: cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

## Building the Project

### Build All Utilities

```bash
# Navigate to project directory
cd E:\Visual_Studio\CODES\RustCLI

# Build all binaries in release mode
cargo build --release

# Build with all features enabled
cargo build --release --all-features
```

The compiled binaries will be in `target/release/`:
- `echo.exe` (or `echo` on Unix)
- `cat.exe`
- `ls.exe`

### Build Individual Utilities

```bash
# Build only echo
cargo build --release -p echo

# Build only cat
cargo build --release -p cat

# Build only ls with color feature
cargo build --release -p ls --features color
```

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Tests for Specific Crate

```bash
cargo test -p echo
cargo test -p cat
cargo test -p ls
cargo test -p common
```

### Run Tests with Verbose Output

```bash
cargo test -- --nocapture
```

## Code Quality Checks

### Format Code

```bash
# Check formatting
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all
```

### Run Clippy (Linter)

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Check Without Building

```bash
cargo check --all-targets --all-features
```

## Installing the Utilities

### Install to Cargo Bin Directory

The utilities will be installed to `~/.cargo/bin/` (or `%USERPROFILE%\.cargo\bin\` on Windows).

```bash
# Install all
cargo install --path crates/echo
cargo install --path crates/cat
cargo install --path crates/ls

# Make sure ~/.cargo/bin is in your PATH
```

### Add to PATH (if needed)

#### Windows (PowerShell)

```powershell
$env:Path += ";$env:USERPROFILE\.cargo\bin"
```

To make it permanent, add to your PowerShell profile:
```powershell
notepad $PROFILE
# Add the line: $env:Path += ";$env:USERPROFILE\.cargo\bin"
```

#### Linux/macOS

Add to your `~/.bashrc` or `~/.zshrc`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Then reload:
```bash
source ~/.bashrc  # or source ~/.zshrc
```

## Development Workflow

### Quick Development Cycle

```bash
# 1. Make changes to code

# 2. Check if it compiles
cargo check -p <crate-name>

# 3. Run tests
cargo test -p <crate-name>

# 4. Format code
cargo fmt --all

# 5. Run clippy
cargo clippy -p <crate-name>

# 6. Build release version
cargo build --release -p <crate-name>
```

### Running During Development

You can run binaries without installing:

```bash
# Run echo
cargo run -p echo -- hello world

# Run cat
cargo run -p cat -- file.txt

# Run ls
cargo run -p ls -- -la
```

## Troubleshooting

### Cargo Command Not Found

If you get "cargo: command not found" or "cargo is not recognized":

1. Ensure Rust is installed (see Prerequisites)
2. Restart your terminal
3. Verify `%USERPROFILE%\.cargo\bin` is in your PATH

### Build Errors

If you encounter build errors:

1. Update Rust to the latest stable version:
   ```bash
   rustup update stable
   ```

2. Clean the build cache:
   ```bash
   cargo clean
   ```

3. Try building again:
   ```bash
   cargo build
   ```

### Tests Failing

If tests fail:

1. Make sure you're running from the workspace root
2. Check that temp directories are writable
3. On Windows, ensure you have proper permissions
4. Try running individual test suites:
   ```bash
   cargo test -p echo -- --nocapture
   ```

## IDE Setup

### Visual Studio Code

Recommended extensions:

1. **rust-analyzer**: Best Rust language server
2. **CodeLLDB**: Debugger for Rust
3. **Better TOML**: TOML file syntax highlighting
4. **Error Lens**: Inline error display

Install rust-analyzer:
```bash
rustup component add rust-analyzer
```

### Configuration

Create `.vscode/settings.json` in the workspace:

```json
{
  "rust-analyzer.cargo.allFeatures": true,
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

## Next Steps

After setup:

1. Build the project: `cargo build --release`
2. Run tests: `cargo test`
3. Try the utilities: `cargo run -p echo -- hello world`
4. Read the main README.md for usage examples
5. Check individual crate READMEs for detailed documentation

## Additional Resources

- Rust Book: https://doc.rust-lang.org/book/
- Cargo Book: https://doc.rust-lang.org/cargo/
- Rust By Example: https://doc.rust-lang.org/rust-by-example/
- This Project's README: `README.md`

