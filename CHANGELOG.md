# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Find utility implementation
- Grep utility implementation
- Parallel processing support
- Colorized output enhancements
- Shell completions
- Man page generation

## [0.1.0] - 2025-10-30

### Project Overview

This release is a complete Rust recreation of the [Java CLI-Custom project](https://github.com/Zyrex24/CLI-Custom), implementing all core functionality with Rust's systems programming capabilities.

### Added

#### Interactive Shell
- Full REPL (Read-Eval-Print Loop) environment
- Persistent working directory across commands
- Output redirection (`>`, `>>`) matching Java implementation
- Command piping (`|`) for chaining commands
- Interactive command prompt with current directory display
- Help system with comprehensive command documentation

#### PWD Utility
- Print current working directory
- `-L` flag for logical path
- `-P` flag for physical path (resolving symlinks)

#### MKDIR Utility
- Create single or multiple directories
- `-p` flag to create parent directories as needed
- `-v` verbose mode for directory creation feedback

#### RMDIR Utility
- Remove empty directories
- `-p` flag to remove parent directories
- `-v` verbose mode
- `--ignore-fail-on-non-empty` for graceful error handling

#### TOUCH Utility
- Create empty files
- Update file timestamps
- `-c` flag to not create missing files
- Support for multiple files in single command

#### MV Utility
- Move and rename files/directories
- `-f` flag to force overwrite
- `-n` flag to prevent overwriting
- `-v` verbose mode
- Support for multiple source files to directory

#### RM Utility
- Remove files and directories
- `-r/-R` recursive removal of directories
- `-f` force removal without prompting
- `-v` verbose mode
- `-d` flag for empty directory removal

#### Echo Utility
- Basic echo functionality with space-separated arguments
- `-n, --no-newline` flag to suppress trailing newline
- `-e, --escape` flag to enable backslash escape interpretation
- `-E, --no-escape` flag to explicitly disable escapes
- Support for escape sequences: `\n`, `\t`, `\r`, `\\`, `\a`, `\b`, `\f`, `\v`, `\0`
- Comprehensive unit and integration tests

#### Cat Utility
- File concatenation and display
- Stdin support with `-` argument
- `-n, --number` flag to number all lines
- `-b, --number-nonblank` flag to number non-empty lines only
- `-A, --show-all` flag to display non-printing characters
- `-s, --squeeze-blank` flag to squeeze multiple blank lines
- Proper handling of binary files
- Buffered I/O for performance

#### Ls Utility
- Directory and file listing
- `-l, --long` flag for detailed format with permissions and timestamps
- `-a, --all` flag to show hidden files
- `-h, --human-readable` flag for human-readable file sizes
- `-t, --time` flag to sort by modification time
- `-r, --reverse` flag to reverse sort order
- Cross-platform permission display
- Human-readable size formatting (B, K, M, G, T)

#### Common Library
- Shared error types using thiserror
- I/O helper functions for buffered reading/writing
- Optional color module (gated behind feature flag)
- Testing utilities for creating fixtures
- Cross-platform compatibility

#### Infrastructure
- Workspace-based project structure
- Comprehensive test suite with assert_cmd
- CI/CD pipeline with GitHub Actions
  - Format checking
  - Clippy linting
  - Multi-platform testing (Ubuntu, macOS, Windows)
  - Release builds
- Documentation
  - Main README with usage examples
  - Individual crate READMEs
  - Setup guide
  - Contributing guidelines
- Code quality tools
  - rustfmt configuration
  - clippy configuration
  - .gitignore

### Technical Details
- Zero unwrap/expect in production code
- Proper error handling with anyhow and thiserror
- Feature flags for optional functionality
- Comprehensive error messages with context
- Cross-platform support (Windows, macOS, Linux)

## Release Notes Format

### Types of Changes
- `Added` for new features
- `Changed` for changes in existing functionality
- `Deprecated` for soon-to-be removed features
- `Removed` for now removed features
- `Fixed` for any bug fixes
- `Security` in case of vulnerabilities

[Unreleased]: https://github.com/Zyrex24/rustcli/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Zyrex24/rustcli/releases/tag/v0.1.0

