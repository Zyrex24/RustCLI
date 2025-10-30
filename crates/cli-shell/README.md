# CLI Shell

An interactive command-line shell implementing all utility commands with support for redirection and piping. This is the main entry point for the Rust recreation of the [Java CLI-Custom project](https://github.com/Zyrex24/CLI-Custom).

## Features

- **Interactive REPL** environment
- **Persistent state** - working directory maintained across commands
- **Output redirection** - `>` (overwrite) and `>>` (append)
- **Command piping** - `|` to chain commands
- **Built-in commands** - all utilities available without external binaries
- **Help system** - comprehensive command documentation

## Usage

### Starting the Shell

```bash
# Run from project root
cargo run -p cli-shell

# Or install and run
cargo install --path crates/cli-shell
cli-shell
```

### Interactive Session

```bash
Rust CLI Shell v0.1.0
A recreation of the Java CLI-Custom project
Type 'help' for available commands, 'exit' to quit

/home/user/project> ls
file1.txt
file2.txt
directory/

/home/user/project> pwd
/home/user/project

/home/user/project> cd directory

/home/user/project/directory> ls
nested.txt

/home/user/project/directory> cd ..

/home/user/project> exit
Goodbye!
```

## Available Commands

### File Commands
- `cat <file...>` - Concatenate and display files
- `touch <file...>` - Create empty files or update timestamps
- `rm [-r] <file...>` - Remove files or directories
- `mv <source> <dest>` - Move or rename files

### Directory Commands
- `ls [-l] [-a] [path]` - List directory contents
- `pwd` - Print working directory
- `cd <directory>` - Change directory
- `mkdir [-p] <dir...>` - Create directories
- `rmdir <dir...>` - Remove empty directories

### Utility Commands
- `echo <text...>` - Display text
- `help` - Show command help
- `exit` - Exit the shell

## Redirection Examples

### Output to File (Overwrite)

```bash
> echo "Hello World" > output.txt
> ls > filelist.txt
> pwd > current_dir.txt
```

### Output to File (Append)

```bash
> echo "Line 1" > file.txt
> echo "Line 2" >> file.txt
> echo "Line 3" >> file.txt
> cat file.txt
Line 1
Line 2
Line 3
```

## Piping Examples

### Simple Pipe

```bash
> ls | cat
file1.txt
file2.txt
```

### Multiple Pipes

```bash
> echo "test data" | cat
test data
```

### Combining with Redirection

```bash
> ls | cat > filelist.txt
```

## Implementation Details

### Command Parsing

The shell parses commands in this order:
1. Check for piping (`|`)
2. Check for redirection (`>`, `>>`)
3. Parse command and arguments
4. Execute and handle output

### State Management

- **Working Directory**: Persistent across commands via `cd`
- **Environment**: Maintained throughout session
- **Command History**: Available for recall

### Error Handling

Errors are displayed but don't terminate the shell:

```bash
> rm nonexistent.txt
Error: Failed to remove 'nonexistent.txt': No such file or directory

> ls
# Shell continues normally
```

## Differences from Standalone Utilities

### Integrated Commands

Commands in the shell are built-in functions, not separate processes:

- **Faster execution** - no process spawning
- **Shared state** - working directory persists
- **Simpler syntax** - no need for binary paths

### Simplified Flags

Some complex flags available in standalone binaries may have simplified versions in the shell for better interactive use.

## Comparison with Java CLI-Custom

| Feature | Java CLI-Custom | Rust CLI-Shell | Status |
|---------|----------------|----------------|--------|
| Interactive REPL | ✅ | ✅ | Matching |
| Redirection (>) | ✅ | ✅ | Matching |
| Append (>>) | ✅ | ✅ | Matching |
| Piping (\|) | ✅ | ✅ | Matching |
| cd command | ✅ | ✅ | Matching |
| All file commands | ✅ | ✅ | Matching |
| Error handling | ✅ | ✅ | Enhanced |

## Architecture

### Command Dispatcher

```rust
fn process_command(input: &str) -> Result<()> {
    // 1. Check for piping
    if input.contains('|') {
        return process_pipe(input);
    }
    
    // 2. Parse redirection
    let (cmd, redirect) = parse_redirection(input);
    
    // 3. Execute command
    let output = execute_single_command(&cmd)?;
    
    // 4. Handle output
    if let Some((file, append)) = redirect {
        write_to_file(&output, &file, append)?;
    } else {
        print!("{}", output);
    }
    
    Ok(())
}
```

### Built-in Commands Module

All commands are implemented in `commands.rs`:

- `help_command()` - Help system
- `pwd_command()` - Print working directory
- `cd_command()` - Change directory
- `ls_command()` - List files
- `cat_command()` - Concatenate files
- `echo_command()` - Echo text
- `mkdir_command()` - Make directory
- `rmdir_command()` - Remove directory
- `touch_command()` - Touch files
- `rm_command()` - Remove files
- `mv_command()` - Move files

## Building

```bash
# Build debug version
cargo build -p cli-shell

# Build release version
cargo build --release -p cli-shell

# Run tests
cargo test -p cli-shell
```

## Dependencies

- `clap` - Not used in shell mode, but available for future enhancements
- `anyhow` - Error handling
- `common` - Shared utilities
- `dirs` - Home directory support for cd command

## Future Enhancements

- Command history with up/down arrows
- Tab completion
- Colored output for different file types
- Configuration file support
- Script execution mode
- Alias support

## License

Licensed under the Apache License, Version 2.0. Same as the original Java CLI-Custom project.

## Credits

This shell is a Rust recreation of the [Java CLI-Custom project](https://github.com/Zyrex24/CLI-Custom), maintaining feature parity while showcasing Rust's systems programming capabilities.

