use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "rm")]
#[command(about = "Remove files or directories", long_about = None)]
#[command(version)]
struct Args {
    /// Remove directories and their contents recursively
    #[arg(short = 'r', short_alias = 'R', long = "recursive")]
    recursive: bool,

    /// Force removal without prompting
    #[arg(short = 'f', long = "force")]
    force: bool,

    /// Verbose mode
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Remove empty directories
    #[arg(short = 'd', long = "dir")]
    dir: bool,

    /// Files or directories to remove
    #[arg(required = true)]
    files: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    for file in &args.files {
        match remove_path(file, &args) {
            Ok(_) => {}
            Err(e) => {
                if !args.force {
                    return Err(e).with_context(|| format!("Failed to remove '{}'", file));
                }
                // With -f, silently ignore errors
            }
        }
    }
    
    Ok(())
}

fn remove_path(path: &str, args: &Args) -> Result<()> {
    let path_obj = Path::new(path);
    
    if !path_obj.exists() {
        if args.force {
            return Ok(()); // Silently succeed with -f flag
        }
        anyhow::bail!("cannot remove '{}': No such file or directory", path);
    }
    
    if path_obj.is_dir() {
        if args.recursive {
            // Recursively remove directory and contents
            fs::remove_dir_all(path_obj)?;
            
            if args.verbose {
                println!("removed directory '{}'", path);
            }
        } else if args.dir {
            // Remove empty directory only
            match fs::remove_dir(path_obj) {
                Ok(_) => {
                    if args.verbose {
                        println!("removed directory '{}'", path);
                    }
                }
                Err(_) => {
                    anyhow::bail!("cannot remove '{}': Directory not empty", path);
                }
            }
        } else {
            anyhow::bail!("cannot remove '{}': Is a directory", path);
        }
    } else {
        // Remove file
        fs::remove_file(path_obj)?;
        
        if args.verbose {
            println!("removed '{}'", path);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;

    #[test]
    fn test_remove_file() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_rm_file.txt");
        
        File::create(&test_file).unwrap();
        assert!(test_file.exists());
        
        let args = Args {
            recursive: false,
            force: false,
            verbose: false,
            dir: false,
            files: vec![],
        };
        
        let result = remove_path(test_file.to_str().unwrap(), &args);
        assert!(result.is_ok());
        assert!(!test_file.exists());
    }

    #[test]
    fn test_remove_directory_without_r_fails() {
        let temp_dir = env::temp_dir();
        let test_dir = temp_dir.join("test_rm_dir");
        
        fs::create_dir(&test_dir).unwrap();
        
        let args = Args {
            recursive: false,
            force: false,
            verbose: false,
            dir: false,
            files: vec![],
        };
        
        let result = remove_path(test_dir.to_str().unwrap(), &args);
        assert!(result.is_err());
        
        // Cleanup
        fs::remove_dir(&test_dir).unwrap();
    }

    #[test]
    fn test_remove_directory_recursively() {
        let temp_dir = env::temp_dir();
        let test_dir = temp_dir.join("test_rm_recursive");
        
        fs::create_dir(&test_dir).unwrap();
        File::create(test_dir.join("file.txt")).unwrap();
        
        let args = Args {
            recursive: true,
            force: false,
            verbose: false,
            dir: false,
            files: vec![],
        };
        
        let result = remove_path(test_dir.to_str().unwrap(), &args);
        assert!(result.is_ok());
        assert!(!test_dir.exists());
    }

    #[test]
    fn test_remove_nonexistent_with_force() {
        let args = Args {
            recursive: false,
            force: true,
            verbose: false,
            dir: false,
            files: vec![],
        };
        
        let result = remove_path("/nonexistent_file_12345.txt", &args);
        assert!(result.is_ok()); // Should succeed with -f flag
    }
}

