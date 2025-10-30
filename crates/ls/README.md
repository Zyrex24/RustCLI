# Ls

A Rust implementation of the ls (list) utility for displaying directory contents.

## Features

- List files and directories
- Long format with permissions, size, and timestamps
- Human-readable file sizes
- Sort by name or modification time
- Show hidden files
- Cross-platform support

## Usage

```bash
# List current directory
ls

# List specific directory
ls /path/to/directory

# List specific file
ls file.txt

# List multiple paths
ls dir1/ dir2/ file.txt
```

## Command-Line Options

- `-l, --long`: Use long listing format
- `-a, --all`: Show hidden files (those starting with .)
- `-h, --human-readable`: Show file sizes in human-readable format (1K, 234M, 2G)
- `-t, --time`: Sort by modification time, newest first
- `-r, --reverse`: Reverse the sort order

## Examples

### Basic Listing

```bash
ls
# file1.txt
# file2.txt
# directory/
```

### Long Format

```bash
ls -l
# -rw-r--r--      1024 2024-01-15 14:30 file1.txt
# -rw-r--r--      2048 2024-01-15 14:31 file2.txt
# drwxr-xr-x      4096 2024-01-15 14:25 directory
```

### Human-Readable Sizes

```bash
ls -lh
# -rw-r--r--      1.0K 2024-01-15 14:30 file1.txt
# -rw-r--r--      2.0K 2024-01-15 14:31 file2.txt
# drwxr-xr-x      4.0K 2024-01-15 14:25 directory
```

### Show Hidden Files

```bash
ls -a
# .
# ..
# .hidden
# file1.txt
# file2.txt
```

### Sort by Time

```bash
ls -lt
# (files listed with newest first)
```

### Reverse Sort

```bash
ls -lr
# (files listed in reverse alphabetical order)
```

### Combined Options

```bash
# Long format, human-readable, show all, sort by time
ls -laht

# Long format, reverse sort
ls -lr
```

## Long Format Fields

The long format (`-l`) displays the following information:

```
-rw-r--r--      1024 2024-01-15 14:30 file.txt
│           │        │                 │
│           │        │                 └─ File name
│           │        └─ Modification timestamp
│           └─ File size (bytes or human-readable with -h)
└─ Permissions string
```

### Permissions String

The permissions string format:

```
drwxr-xr-x
│└┬┘└┬┘└┬┘
│ │  │  └─ Other permissions (read, write, execute)
│ │  └─ Group permissions
│ └─ Owner permissions
└─ File type (d=directory, l=symlink, -=regular file)
```

Permissions characters:
- `r`: read permission
- `w`: write permission
- `x`: execute permission
- `-`: permission not granted

## Human-Readable Sizes

When using `-h`, file sizes are displayed with these units:

- `B`: Bytes (0-1023 bytes)
- `K`: Kilobytes (1024 bytes)
- `M`: Megabytes (1024 KB)
- `G`: Gigabytes (1024 MB)
- `T`: Terabytes (1024 GB)

## Building

```bash
# Build
cargo build --release

# Build with color support
cargo build --release --features color

# Install
cargo install --path .

# Run tests
cargo test
```

## Implementation Details

- Uses standard library filesystem APIs
- Cross-platform path handling
- Efficient directory traversal
- Graceful error handling for permission issues
- Platform-specific permission formatting

## Platform Differences

**Unix/Linux/macOS:**
- Full permission string display (rwxr-xr-x)
- Accurate file type detection (regular, directory, symlink)

**Windows:**
- Simplified permission display
- Directory type detection works correctly
- Symlink detection supported

## Error Handling

Clear error messages for:
- Non-existent paths
- Permission denied
- Invalid directory access

Errors don't stop processing of multiple paths - each path is handled independently.

## Sorting Behavior

**Alphabetical (default):**
- Case-insensitive sorting
- Directories mixed with files

**Time-based (`-t`):**
- Sorted by modification time
- Newest files first (use `-r` to reverse)

**Reverse (`-r`):**
- Reverses the current sort order
- Can be combined with `-t`

## License

Licensed under the Apache License, Version 2.0.

