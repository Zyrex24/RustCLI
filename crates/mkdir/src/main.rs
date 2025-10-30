use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "mkdir")]
#[command(about = "Create directories", long_about = None)]
#[command(version)]
struct Args {
    /// Create parent directories as needed
    #[arg(short = 'p', long = "parents")]
    parents: bool,

    /// Verbose mode - print a message for each created directory
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Directories to create
    #[arg(required = true)]
    directories: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    for dir in &args.directories {
        create_directory(dir, args.parents, args.verbose)
            .with_context(|| format!("Failed to create directory: {}", dir))?;
    }
    
    Ok(())
}

fn create_directory(path: &str, create_parents: bool, verbose: bool) -> Result<()> {
    let path_obj = Path::new(path);
    
    // Check if directory already exists
    if path_obj.exists() {
        if !create_parents {
            anyhow::bail!("cannot create directory '{}': File exists", path);
        }
        // With -p flag, silently succeed if directory exists
        return Ok(());
    }
    
    if create_parents {
        fs::create_dir_all(path_obj)?;
    } else {
        fs::create_dir(path_obj)?;
    }
    
    if verbose {
        println!("created directory '{}'", path);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_create_single_directory() {
        let temp_dir = env::temp_dir();
        let test_dir = temp_dir.join("test_mkdir_single");
        
        // Clean up if exists
        let _ = fs::remove_dir(&test_dir);
        
        let result = create_directory(test_dir.to_str().unwrap(), false, false);
        assert!(result.is_ok());
        assert!(test_dir.exists());
        
        // Cleanup
        fs::remove_dir(&test_dir).unwrap();
    }

    #[test]
    fn test_create_with_parents() {
        let temp_dir = env::temp_dir();
        let test_dir = temp_dir.join("test_mkdir_parent").join("subdir").join("deep");
        
        // Clean up if exists
        let _ = fs::remove_dir_all(&test_dir);
        
        let result = create_directory(test_dir.to_str().unwrap(), true, false);
        assert!(result.is_ok());
        assert!(test_dir.exists());
        
        // Cleanup
        fs::remove_dir_all(temp_dir.join("test_mkdir_parent")).unwrap();
    }

    #[test]
    fn test_create_existing_directory_without_p() {
        let temp_dir = env::temp_dir();
        let test_dir = temp_dir.join("test_mkdir_existing");
        
        // Create the directory first
        let _ = fs::create_dir(&test_dir);
        
        let result = create_directory(test_dir.to_str().unwrap(), false, false);
        assert!(result.is_err());
        
        // Cleanup
        fs::remove_dir(&test_dir).unwrap();
    }
}

