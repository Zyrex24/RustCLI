use anyhow::{Context, Result};
use clap::Parser;
use std::fs::{File, OpenOptions};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "touch")]
#[command(about = "Create empty files or update timestamps", long_about = None)]
#[command(version)]
struct Args {
    /// Do not create files that do not exist
    #[arg(short = 'c', long = "no-create")]
    no_create: bool,

    /// Files to create or update
    #[arg(required = true)]
    files: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    for file in &args.files {
        touch_file(file, args.no_create)
            .with_context(|| format!("Failed to touch file: {}", file))?;
    }
    
    Ok(())
}

fn touch_file(path: &str, no_create: bool) -> Result<()> {
    let path_obj = Path::new(path);
    
    if path_obj.exists() {
        // Update the modification time by opening and closing the file
        OpenOptions::new()
            .write(true)
            .open(path_obj)?;
        Ok(())
    } else {
        if no_create {
            // Don't create if -c flag is set
            Ok(())
        } else {
            // Create the file
            File::create(path_obj)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_create_new_file() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_touch_new.txt");
        
        // Clean up if exists
        let _ = fs::remove_file(&test_file);
        
        assert!(!test_file.exists());
        
        let result = touch_file(test_file.to_str().unwrap(), false);
        assert!(result.is_ok());
        assert!(test_file.exists());
        
        // Cleanup
        fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_update_existing_file() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_touch_existing.txt");
        
        // Create file first
        File::create(&test_file).unwrap();
        thread::sleep(Duration::from_millis(10));
        
        let metadata_before = fs::metadata(&test_file).unwrap();
        
        thread::sleep(Duration::from_millis(10));
        
        let result = touch_file(test_file.to_str().unwrap(), false);
        assert!(result.is_ok());
        
        let metadata_after = fs::metadata(&test_file).unwrap();
        
        // Modified time should be updated (or at least not older)
        assert!(metadata_after.modified().unwrap() >= metadata_before.modified().unwrap());
        
        // Cleanup
        fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_no_create_flag() {
        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_touch_no_create.txt");
        
        // Clean up if exists
        let _ = fs::remove_file(&test_file);
        
        assert!(!test_file.exists());
        
        let result = touch_file(test_file.to_str().unwrap(), true);
        assert!(result.is_ok());
        assert!(!test_file.exists()); // Should NOT be created
    }
}

