# Echo

A Rust implementation of the echo utility that displays text to standard output.

## Features

- Print text arguments to stdout
- Optional newline suppression
- Backslash escape sequence interpretation
- Zero dependencies beyond argument parsing

## Usage

```bash
# Basic echo
echo hello world
# Output: hello world

# Suppress trailing newline
echo -n "prompt: "
# Output: prompt: (no newline)

# Interpret escape sequences
echo -e "Line 1\nLine 2\tTabbed"
# Output:
# Line 1
# Line 2    Tabbed

# Multiple arguments are space-separated
echo one two three
# Output: one two three
```

## Command-Line Options

- `-n, --no-newline`: Do not output the trailing newline
- `-e, --escape`: Enable interpretation of backslash escapes
- `-E, --no-escape`: Disable interpretation of backslash escapes (default)

## Supported Escape Sequences

When `-e` is enabled, the following escape sequences are interpreted:

- `\n`: newline
- `\t`: horizontal tab
- `\r`: carriage return
- `\\`: backslash
- `\a`: alert (bell)
- `\b`: backspace
- `\f`: form feed
- `\v`: vertical tab
- `\0`: null character

## Examples

```bash
# Bell sound
echo -e "\a"

# Formatted table
echo -e "Name\tAge\nAlice\t30\nBob\t25"

# No newline for prompts
echo -n "Enter your name: "
```

## Implementation Details

- Pure Rust implementation using standard library
- Proper error handling with anyhow
- Manual escape sequence parsing for control
- No unwrap() or expect() calls

## Building

```bash
# Build
cargo build --release

# Install
cargo install --path .

# Run tests
cargo test
```

## Differences from GNU Echo

This implementation follows POSIX echo behavior with the following notes:

- The `-E` flag explicitly disables escape interpretation
- When both `-e` and `-E` are specified, `-E` takes precedence
- Unrecognized escape sequences are left as-is (e.g., `\x` stays as `\x`)

## License

Licensed under the Apache License, Version 2.0.

