# Rust CLI Utilities

A workspace containing modern Rust implementations of classic command-line utilities. This project is a **Rust recreation** of the [Java CLI-Custom project](https://github.com/Zyrex24/CLI-Custom), demonstrating how the same functionality can be implemented with Rust's systems programming capabilities, proper error handling, cross-platform support, and comprehensive testing.

## Inspiration

This project recreates the functionality of the [CLI-Custom Java project](https://github.com/Zyrex24/CLI-Custom) in Rust, maintaining feature parity while leveraging Rust's unique strengths: memory safety, zero-cost abstractions, and fearless concurrency.

## Project Structure

```
rustcli/
├── crates/
│   ├── common/          # Shared utilities library
│   ├── echo/            # Echo utility
│   ├── cat/             # File concatenation utility
│   ├── ls/              # Directory listing utility
│   ├── pwd/             # Print working directory
│   ├── mkdir/           # Create directories
│   ├── rmdir/           # Remove empty directories
│   ├── touch/           # Create empty files
│   ├── mv/              # Move/rename files
│   ├── rm/              # Remove files and directories
│   ├── cli-shell/       # Interactive shell with redirection & piping
│   ├── find/            # File search utility (planned)
│   └── grep/            # Text pattern matching utility (planned)
├── Cargo.toml           # Workspace configuration
└── README.md
```

## Features

**Current Implementation (v0.1.0)** - Matching Java CLI-Custom functionality

**File Commands:**
- **cat**: Concatenate and display files with line numbering and formatting options
- **touch**: Create empty files or update timestamps
- **rm**: Remove files or directories (with `-r` for recursive removal)
- **mv**: Move or rename files and directories

**Directory Commands:**
- **ls**: List directory contents with long format, sorting, and human-readable sizes
- **pwd**: Print working directory (current path)
- **mkdir**: Create directories (with `-p` for parent creation)
- **rmdir**: Remove empty directories

**Utility Commands:**
- **echo**: Display text with optional escape sequence interpretation
- **help**: Display help information for all commands

**Interactive Shell Features:**
- **Redirection**: `>` (overwrite) and `>>` (append) output to files
- **Piping**: `|` to pass output from one command to another
- **REPL**: Interactive command-line interface with persistent state

**Planned (v0.2+)**

- **find**: Locate files and directories by name, type, and other criteria
- **grep**: Search for patterns in files with regex support and parallel processing

## Installation

### Building from Source

```bash
# Clone the repository
git clone https://github.com/Zyrex24/rustcli.git
cd rustcli

# Build all utilities
cargo build --release

# Install the interactive shell (recommended)
cargo install --path crates/cli-shell

# Or install individual utilities
cargo install --path crates/echo
cargo install --path crates/cat
cargo install --path crates/ls
cargo install --path crates/pwd
cargo install --path crates/mkdir
cargo install --path crates/rmdir
cargo install --path crates/touch
cargo install --path crates/mv
cargo install --path crates/rm
```

The binaries will be installed to `~/.cargo/bin/` (or `%USERPROFILE%\.cargo\bin\` on Windows).

### Running the Interactive Shell

```bash
# Run the interactive shell
cli-shell

# Or directly from the project
cargo run -p cli-shell
```

### Building Individual Tools

```bash
# Build only echo
cargo build --release -p echo

# Build with optional features
cargo build --release -p ls --features color
```

## Usage

### Interactive Shell

The CLI shell provides an interactive environment with all commands available:

```bash
cli-shell
```

**Features:**
- Persistent working directory across commands
- Output redirection (`>`, `>>`)
- Command piping (`|`)
- All commands available without prefixes

**Example Session:**
```bash
Rust CLI Shell v0.1.0
A recreation of the Java CLI-Custom project
Type 'help' for available commands, 'exit' to quit

/home/user> ls
file1.txt
file2.txt

/home/user> echo "Hello World" > output.txt

/home/user> cat output.txt
Hello World

/home/user> ls | cat
file1.txt
file2.txt
output.txt

/home/user> exit
Goodbye!
```

### Echo

Display text with optional formatting.

```bash
# Basic usage
echo hello world

# Suppress newline
echo -n "no newline"

# Interpret escape sequences
echo -e "line1\nline2\ttabbed"

# Disable escape interpretation (default)
echo -E "literal\ntext"
```

**Flags:**
- `-n, --no-newline`: Suppress trailing newline
- `-e, --escape`: Enable interpretation of backslash escapes
- `-E, --no-escape`: Disable interpretation (default)

**Supported Escapes:**
- `\n`: newline
- `\t`: tab
- `\r`: carriage return
- `\\`: backslash
- `\a`: alert (bell)
- `\b`: backspace
- `\f`: form feed
- `\v`: vertical tab
- `\0`: null

### Cat

Concatenate and display files.

```bash
# Display a file
cat file.txt

# Display multiple files
cat file1.txt file2.txt

# Read from stdin
cat -
echo "hello" | cat -

# Number all lines
cat -n file.txt

# Number non-blank lines only
cat -b file.txt

# Show all non-printing characters
cat -A file.txt

# Squeeze multiple blank lines
cat -s file.txt
```

**Flags:**
- `-n, --number`: Number all output lines
- `-b, --number-nonblank`: Number non-empty lines only
- `-A, --show-all`: Show all characters (tabs as ^I, etc.)
- `-s, --squeeze-blank`: Squeeze multiple adjacent blank lines

### Ls

List directory contents.

```bash
# List current directory
ls

# List specific directory
ls /path/to/dir

# Long format with details
ls -l

# Show hidden files
ls -a

# Human-readable file sizes
ls -lh

# Sort by modification time
ls -t

# Reverse sort order
ls -r

# Combine flags
ls -lah
```

**Flags:**
- `-l, --long`: Use long listing format
- `-a, --all`: Show hidden files (starting with .)
- `-h, --human-readable`: Print sizes in human-readable format (1K, 234M, 2G)
- `-t, --time`: Sort by modification time (newest first)
- `-r, --reverse`: Reverse sort order

### PWD

Print the current working directory.

```bash
# Display current directory
pwd

# With physical path (resolving symlinks)
pwd -P

# With logical path
pwd -L
```

### MKDIR

Create directories.

```bash
# Create a single directory
mkdir new_directory

# Create multiple directories
mkdir dir1 dir2 dir3

# Create parent directories as needed
mkdir -p path/to/nested/directory

# Verbose output
mkdir -v new_dir
```

**Flags:**
- `-p, --parents`: Create parent directories as needed
- `-v, --verbose`: Print a message for each created directory

### RMDIR

Remove empty directories.

```bash
# Remove an empty directory
rmdir empty_dir

# Remove parent directories
rmdir -p path/to/empty/directory

# Ignore failures on non-empty directories
rmdir --ignore-fail-on-non-empty directory
```

**Flags:**
- `-p, --parents`: Remove parent directories as needed
- `-v, --verbose`: Print a message for each removed directory
- `--ignore-fail-on-non-empty`: Ignore failures on non-empty directories

### TOUCH

Create empty files or update file timestamps.

```bash
# Create a new file
touch newfile.txt

# Create multiple files
touch file1.txt file2.txt file3.txt

# Update timestamp of existing file
touch existing_file.txt

# Do not create file if it doesn't exist
touch -c maybe_exists.txt
```

**Flags:**
- `-c, --no-create`: Do not create files that do not exist

### MV

Move or rename files and directories.

```bash
# Rename a file
mv oldname.txt newname.txt

# Move file to directory
mv file.txt /path/to/directory/

# Move multiple files to directory
mv file1.txt file2.txt /path/to/directory/

# Force overwrite without prompting
mv -f source.txt destination.txt

# Do not overwrite existing files
mv -n source.txt destination.txt

# Verbose output
mv -v old.txt new.txt
```

**Flags:**
- `-f, --force`: Do not prompt before overwriting
- `-n, --no-clobber`: Do not overwrite existing files
- `-v, --verbose`: Explain what is being done

### RM

Remove files and directories.

```bash
# Remove a file
rm file.txt

# Remove multiple files
rm file1.txt file2.txt file3.txt

# Remove directory and contents recursively
rm -r directory/

# Force removal without prompting
rm -f file.txt

# Verbose output
rm -v file.txt

# Remove empty directory
rm -d empty_dir/

# Combine flags
rm -rf directory/
```

**Flags:**
- `-r, -R, --recursive`: Remove directories and their contents recursively
- `-f, --force`: Force removal without prompting, ignore nonexistent files
- `-v, --verbose`: Explain what is being done
- `-d, --dir`: Remove empty directories

### Redirection and Piping

The interactive shell supports output redirection and command piping, matching the Java CLI-Custom functionality.

**Redirection:**
```bash
# Overwrite file with output
echo "Hello" > output.txt
ls > filelist.txt

# Append to file
echo "World" >> output.txt
ls >> filelist.txt
```

**Piping:**
```bash
# Pipe output to another command
ls | cat
echo "test" | cat
```

## Feature Flags

Optional features can be enabled at compile time:

**Common Library:**
- `color`: Enable colored output support

**Echo:**
- `completions`: Generate shell completions

**Cat:**
- `color`: Enable colored output (inherits from common)

**Ls:**
- `color`: Enable colored directory listings

Example:
```bash
cargo build --release -p ls --features color
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p echo
cargo test -p cat
cargo test -p ls

# Run integration tests only
cargo test --test integration_tests
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Run clippy lints
cargo clippy --all-targets --all-features -- -D warnings

# Check without building
cargo check --all-targets --all-features
```

### Project Guidelines

**Error Handling:**
- Use `anyhow::Result` in binary crates for application errors
- Use `thiserror` in the common library for structured errors
- No `.unwrap()` or `.expect()` in production code paths
- Proper error context using `.context()` from anyhow

**Testing:**
- Unit tests for core logic functions
- Integration tests using `assert_cmd` for CLI testing
- Test edge cases: empty input, invalid UTF-8, permission errors
- Cross-platform testing on Windows, macOS, and Linux

**Code Style:**
- Follow Rust standard naming conventions
- Use `cargo fmt` for consistent formatting
- Address all clippy warnings
- Document public APIs with doc comments

## Roadmap

**v0.1.0 (Current)**
- Core implementations of echo, cat, and ls
- Basic flag support and options
- Comprehensive test coverage
- Cross-platform compatibility

**v0.2.0 (Planned)**
- Implement find utility with name/type/depth filtering
- Implement grep utility with regex support
- Add basic colorized output

**v0.3.0 (Planned)**
- Performance optimizations
- Parallel processing with rayon for grep and find
- Extended ls color schemes
- Additional flags and options

**v0.4.0 (Planned)**
- Shell completions for all utilities
- Man page generation
- Prebuilt binaries for releases
- Packaging (Homebrew, cargo-binstall)

## Technical Details

**Architecture:**
- Workspace-based project structure for code sharing
- Common library for shared utilities (I/O, errors, colors)
- Individual binary crates for each tool
- Feature flags for optional functionality

**Dependencies:**
- `clap`: Command-line argument parsing with derive macros
- `anyhow`: Flexible error handling for applications
- `thiserror`: Structured error types for libraries
- `colored`: Terminal color support (optional)

**Testing Stack:**
- `assert_cmd`: Integration testing for CLI applications
- `predicates`: Assertion helpers for testing
- `tempfile`: Temporary file creation for tests

## Contributing

Contributions are welcome. Please ensure:

1. Code passes `cargo fmt` and `cargo clippy`
2. All tests pass on your platform
3. New features include tests
4. Public APIs have documentation comments

## License

Licensed under the Apache License, Version 2.0. See LICENSE file for details.

## Acknowledgments

Inspired by GNU coreutils and the Rust community's focus on building reliable, performant systems software.

