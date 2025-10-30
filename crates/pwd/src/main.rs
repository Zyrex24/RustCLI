use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(name = "pwd")]
#[command(about = "Print the current working directory", long_about = None)]
#[command(version)]
struct Args {
    /// Use logical path (follow symlinks)
    #[arg(short = 'L', long)]
    logical: bool,

    /// Use physical path (resolve symlinks)
    #[arg(short = 'P', long)]
    physical: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let current_dir = get_current_directory(&args)?;
    print_directory(&current_dir)?;
    
    Ok(())
}

fn get_current_directory(args: &Args) -> Result<String> {
    let path = if args.physical {
        // Get the canonicalized (physical) path
        env::current_dir()
            .context("Failed to get current directory")?
            .canonicalize()
            .context("Failed to canonicalize path")?
    } else {
        // Get the logical path (default)
        env::current_dir()
            .context("Failed to get current directory")?
    };
    
    path.to_str()
        .ok_or_else(|| anyhow::anyhow!("Path contains invalid UTF-8"))
        .map(|s| s.to_string())
}

fn print_directory(dir: &str) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    writeln!(handle, "{}", dir)?;
    handle.flush()?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_directory() {
        let args = Args {
            logical: false,
            physical: false,
        };
        
        let result = get_current_directory(&args);
        assert!(result.is_ok());
        
        let dir = result.unwrap();
        assert!(!dir.is_empty());
    }

    #[test]
    fn test_print_directory() {
        let result = print_directory("/test/path");
        assert!(result.is_ok());
    }
}

