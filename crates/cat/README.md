# Cat

A Rust implementation of the cat (concatenate) utility for reading and displaying file contents.

## Features

- Concatenate multiple files to stdout
- Read from stdin when no files specified or `-` used
- Line numbering (all lines or non-blank only)
- Show non-printing characters
- Squeeze consecutive blank lines
- Robust binary file handling

## Usage

```bash
# Display file contents
cat file.txt

# Concatenate multiple files
cat file1.txt file2.txt file3.txt

# Read from stdin
cat -
echo "hello" | cat -

# Mix files and stdin
cat header.txt - footer.txt < input.txt
```

## Command-Line Options

- `-n, --number`: Number all output lines
- `-b, --number-nonblank`: Number non-empty lines only (overrides -n)
- `-A, --show-all`: Show all non-printing characters
- `-s, --squeeze-blank`: Squeeze multiple adjacent blank lines into one

## Examples

### Line Numbering

```bash
# Number all lines
cat -n file.txt
#      1  first line
#      2  second line
#      3  
#      4  fourth line

# Number only non-blank lines
cat -b file.txt
#      1  first line
#      2  second line
#       
#      3  fourth line
```

### Show Non-Printing Characters

```bash
# Display tabs and special characters
cat -A file.txt
# hello^Iworld$
# (tab shown as ^I, line endings as $)
```

### Squeeze Blank Lines

```bash
# Reduce multiple blank lines to one
cat -s file.txt
# line 1
# 
# line 2
# (multiple blanks squeezed to single blank)
```

### Combining Options

```bash
# Number non-blank lines and squeeze blanks
cat -bs file.txt

# Show all characters with line numbers
cat -nA file.txt
```

## Implementation Details

- Efficient buffered I/O for large files
- Byte-oriented processing (handles non-UTF8 files)
- Proper error handling and reporting
- Line-by-line streaming (low memory usage)
- Cross-platform file handling

## Building

```bash
# Build
cargo build --release

# Build with color feature
cargo build --release --features color

# Install
cargo install --path .

# Run tests
cargo test
```

## Error Handling

The utility provides clear error messages for common issues:

- File not found
- Permission denied
- I/O errors during reading

All errors include context about which file caused the problem when processing multiple files.

## Performance

This implementation uses buffered I/O for efficient file reading. For very large files, memory usage remains constant as the file is processed in a streaming fashion.

## Differences from GNU Cat

This implementation follows standard cat behavior with these notes:

- The `-A` flag shows all non-printing characters
- Binary files are handled gracefully without crashes
- Line numbers use right-aligned 6-digit format with tab separator
- Multiple blank line squeezing works across file boundaries

## License

Licensed under the Apache License, Version 2.0.

