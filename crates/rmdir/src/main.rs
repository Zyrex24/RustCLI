use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "rmdir")]
#[command(about = "Remove empty directories", long_about = None)]
#[command(version)]
struct Args {
    /// Remove parent directories as needed
    #[arg(short = 'p', long = "parents")]
    parents: bool,

    /// Verbose mode
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Ignore failures on non-empty directories
    #[arg(long = "ignore-fail-on-non-empty")]
    ignore_fail_on_non_empty: bool,

    /// Directories to remove
    #[arg(required = true)]
    directories: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    for dir in &args.directories {
        match remove_directory(dir, args.parents, args.verbose) {
            Ok(_) => {}
            Err(e) => {
                if !args.ignore_fail_on_non_empty {
                    return Err(e).with_context(|| format!("Failed to remove directory: {}", dir));
                }
            }
        }
    }
    
    Ok(())
}

fn remove_directory(path: &str, remove_parents: bool, verbose: bool) -> Result<()> {
    let path_obj = Path::new(path);
    
    if !path_obj.exists() {
        anyhow::bail!("failed to remove '{}': No such file or directory", path);
    }
    
    if !path_obj.is_dir() {
        anyhow::bail!("failed to remove '{}': Not a directory", path);
    }
    
    // Check if directory is empty
    let is_empty = fs::read_dir(path_obj)?.next().is_none();
    
    if !is_empty {
        anyhow::bail!("failed to remove '{}': Directory not empty", path);
    }
    
    fs::remove_dir(path_obj)?;
    
    if verbose {
        println!("removed directory '{}'", path);
    }
    
    // If -p flag, try to remove parent directories
    if remove_parents {
        if let Some(parent) = path_obj.parent() {
            if parent.as_os_str().is_empty() || parent == Path::new(".") {
                return Ok(());
            }
            
            let parent_str = parent.to_str().ok_or_else(|| {
                anyhow::anyhow!("Parent path contains invalid UTF-8")
            })?;
            
            // Try to remove parent, but don't fail if it's not empty
            let _ = remove_directory(parent_str, true, verbose);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_remove_empty_directory() {
        let temp_dir = env::temp_dir();
        let test_dir = temp_dir.join("test_rmdir_empty");
        
        fs::create_dir(&test_dir).unwrap();
        assert!(test_dir.exists());
        
        let result = remove_directory(test_dir.to_str().unwrap(), false, false);
        assert!(result.is_ok());
        assert!(!test_dir.exists());
    }

    #[test]
    fn test_remove_non_empty_directory_fails() {
        let temp_dir = env::temp_dir();
        let test_dir = temp_dir.join("test_rmdir_non_empty");
        
        fs::create_dir(&test_dir).unwrap();
        fs::File::create(test_dir.join("file.txt")).unwrap();
        
        let result = remove_directory(test_dir.to_str().unwrap(), false, false);
        assert!(result.is_err());
        
        // Cleanup
        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[test]
    fn test_remove_nonexistent_directory_fails() {
        let result = remove_directory("/nonexistent_dir_12345", false, false);
        assert!(result.is_err());
    }
}

