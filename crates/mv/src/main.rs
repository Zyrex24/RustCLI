use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "mv")]
#[command(about = "Move (rename) files", long_about = None)]
#[command(version)]
struct Args {
    /// Do not prompt before overwriting
    #[arg(short = 'f', long = "force")]
    force: bool,

    /// Do not overwrite existing file
    #[arg(short = 'n', long = "no-clobber")]
    no_clobber: bool,

    /// Verbose mode
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Source file(s) or directory
    #[arg(required = true)]
    source: Vec<String>,

    /// Destination file or directory
    #[arg(required = true, last = true)]
    destination: String,
}

fn main() -> Result<()> {
    let mut args = Args::parse();
    
    // Extract destination from source list
    let destination = args.destination;
    
    // If only one source, simple move/rename
    if args.source.len() == 1 {
        move_file(&args.source[0], &destination, args.no_clobber, args.verbose)
            .with_context(|| format!("Failed to move '{}' to '{}'", args.source[0], destination))?;
    } else {
        // Multiple sources - destination must be a directory
        let dest_path = Path::new(&destination);
        if !dest_path.exists() || !dest_path.is_dir() {
            anyhow::bail!("target '{}' is not a directory", destination);
        }
        
        for source in &args.source {
            let source_path = Path::new(source);
            let file_name = source_path.file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid source path: {}", source))?;
            
            let dest_file = dest_path.join(file_name);
            let dest_str = dest_file.to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid destination path"))?;
            
            move_file(source, dest_str, args.no_clobber, args.verbose)
                .with_context(|| format!("Failed to move '{}' to '{}'", source, dest_str))?;
        }
    }
    
    Ok(())
}

fn move_file(source: &str, destination: &str, no_clobber: bool, verbose: bool) -> Result<()> {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);
    
    if !source_path.exists() {
        anyhow::bail!("cannot stat '{}': No such file or directory", source);
    }
    
    // Check if destination exists
    if dest_path.exists() {
        if no_clobber {
            return Ok(()); // Skip if no-clobber is set
        }
        
        // If destination is a directory and source is not, move into directory
        if dest_path.is_dir() && !source_path.is_dir() {
            let file_name = source_path.file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid source path: {}", source))?;
            let new_dest = dest_path.join(file_name);
            return move_file(source, new_dest.to_str().unwrap(), no_clobber, verbose);
        }
    }
    
    fs::rename(source_path, dest_path)?;
    
    if verbose {
        println!("'{}' -> '{}'", source, destination);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_move_file() {
        let temp_dir = env::temp_dir();
        let source = temp_dir.join("test_mv_source.txt");
        let dest = temp_dir.join("test_mv_dest.txt");
        
        // Create source file
        File::create(&source).unwrap();
        
        // Clean up dest if exists
        let _ = fs::remove_file(&dest);
        
        let result = move_file(
            source.to_str().unwrap(),
            dest.to_str().unwrap(),
            false,
            false
        );
        
        assert!(result.is_ok());
        assert!(!source.exists());
        assert!(dest.exists());
        
        // Cleanup
        fs::remove_file(&dest).unwrap();
    }

    #[test]
    fn test_rename_file() {
        let temp_dir = env::temp_dir();
        let source = temp_dir.join("test_mv_rename_old.txt");
        let dest = temp_dir.join("test_mv_rename_new.txt");
        
        // Create source with content
        let mut file = File::create(&source).unwrap();
        writeln!(file, "test content").unwrap();
        
        // Clean up dest if exists
        let _ = fs::remove_file(&dest);
        
        let result = move_file(
            source.to_str().unwrap(),
            dest.to_str().unwrap(),
            false,
            false
        );
        
        assert!(result.is_ok());
        assert!(!source.exists());
        assert!(dest.exists());
        
        // Verify content preserved
        let content = fs::read_to_string(&dest).unwrap();
        assert!(content.contains("test content"));
        
        // Cleanup
        fs::remove_file(&dest).unwrap();
    }

    #[test]
    fn test_move_nonexistent_file() {
        let result = move_file("/nonexistent_12345.txt", "/dest.txt", false, false);
        assert!(result.is_err());
    }
}

