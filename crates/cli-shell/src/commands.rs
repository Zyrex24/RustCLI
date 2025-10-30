use anyhow::Result;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub fn help_command() -> Result<String> {
    let help_text = r#"
Available Commands:
===================

File Commands:
  cat <file...>        - Concatenate and display files
  touch <file...>      - Create empty files or update timestamps
  rm [-r] <file...>    - Remove files or directories
  mv <source> <dest>   - Move or rename files

Directory Commands:
  ls [-l] [-a] [path]  - List directory contents
  pwd                  - Print working directory
  cd <directory>       - Change directory
  mkdir [-p] <dir...>  - Create directories
  rmdir <dir...>       - Remove empty directories

Utility Commands:
  echo <text...>       - Display text
  help                 - Show this help message
  exit                 - Exit the shell

Special Syntax:
  >                    - Redirect output to file (overwrite)
  >>                   - Redirect output to file (append)
  |                    - Pipe output to another command

Examples:
  ls -l
  cat file.txt
  echo "Hello" > output.txt
  ls | cat
  mkdir -p path/to/directory

"#;
    Ok(help_text.to_string())
}

pub fn pwd_command() -> Result<String> {
    let current_dir = env::current_dir()?;
    Ok(format!("{}\n", current_dir.display()))
}

pub fn cd_command(args: &[&str]) -> Result<String> {
    if args.is_empty() {
        // Go to home directory
        if let Some(home) = dirs::home_dir() {
            env::set_current_dir(home)?;
        } else {
            anyhow::bail!("Could not determine home directory");
        }
    } else {
        let path = Path::new(args[0]);
        if !path.exists() {
            anyhow::bail!("cd: {}: No such file or directory", args[0]);
        }
        if !path.is_dir() {
            anyhow::bail!("cd: {}: Not a directory", args[0]);
        }
        env::set_current_dir(path)?;
    }
    Ok(String::new())
}

pub fn ls_command(args: &[&str]) -> Result<String> {
    let mut output = String::new();
    
    let show_all = args.contains(&"-a");
    let long_format = args.contains(&"-l");
    
    let path = args.iter()
        .find(|&&arg| !arg.starts_with('-'))
        .map(|&s| s)
        .unwrap_or(".");
    
    let entries = fs::read_dir(path)?;
    let mut files: Vec<_> = entries.collect::<Result<_, _>>()?;
    files.sort_by_key(|e| e.file_name());
    
    for entry in files {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        
        if !show_all && name.starts_with('.') {
            continue;
        }
        
        if long_format {
            let metadata = entry.metadata()?;
            let size = metadata.len();
            let file_type = if metadata.is_dir() { "d" } else { "-" };
            output.push_str(&format!("{} {:>10} {}\n", file_type, size, name));
        } else {
            output.push_str(&format!("{}\n", name));
        }
    }
    
    Ok(output)
}

pub fn cat_command(args: &[&str]) -> Result<String> {
    let mut output = String::new();
    
    if args.is_empty() {
        // Read from stdin
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            output.push_str(&line?);
            output.push('\n');
        }
    } else {
        for file_name in args {
            if !file_name.starts_with('-') {
                let content = fs::read_to_string(file_name)?;
                output.push_str(&content);
            }
        }
    }
    
    Ok(output)
}

pub fn echo_command(args: &[&str]) -> Result<String> {
    let text = args.join(" ");
    Ok(format!("{}\n", text))
}

pub fn mkdir_command(args: &[&str]) -> Result<String> {
    let parents = args.contains(&"-p");
    
    for arg in args {
        if !arg.starts_with('-') {
            if parents {
                fs::create_dir_all(arg)?;
            } else {
                fs::create_dir(arg)?;
            }
        }
    }
    
    Ok(String::new())
}

pub fn rmdir_command(args: &[&str]) -> Result<String> {
    for arg in args {
        if !arg.starts_with('-') {
            fs::remove_dir(arg)?;
        }
    }
    
    Ok(String::new())
}

pub fn touch_command(args: &[&str]) -> Result<String> {
    for arg in args {
        if !arg.starts_with('-') {
            if !Path::new(arg).exists() {
                fs::File::create(arg)?;
            }
        }
    }
    
    Ok(String::new())
}

pub fn rm_command(args: &[&str]) -> Result<String> {
    let recursive = args.contains(&"-r") || args.contains(&"-R");
    
    for arg in args {
        if !arg.starts_with('-') {
            let path = Path::new(arg);
            if path.is_dir() {
                if recursive {
                    fs::remove_dir_all(path)?;
                } else {
                    anyhow::bail!("rm: {}: is a directory", arg);
                }
            } else {
                fs::remove_file(path)?;
            }
        }
    }
    
    Ok(String::new())
}

pub fn mv_command(args: &[&str]) -> Result<String> {
    if args.len() < 2 {
        anyhow::bail!("mv: missing destination file operand");
    }
    
    let source = args[0];
    let dest = args[1];
    
    fs::rename(source, dest)?;
    
    Ok(String::new())
}

// Note: This requires the dirs crate for home directory support
// Add to Cargo.toml: dirs = "5.0"

