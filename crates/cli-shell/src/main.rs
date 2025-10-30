use anyhow::Result;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::{Command, Stdio};

mod commands;
use commands::*;

fn main() -> Result<()> {
    println!("Rust CLI Shell v0.1.0");
    println!("A recreation of the Java CLI-Custom project");
    println!("Type 'help' for available commands, 'exit' to quit\n");
    
    loop {
        // Print prompt
        let current_dir = env::current_dir()?;
        print!("{}> ", current_dir.display());
        io::stdout().flush()?;
        
        // Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        // Check for exit command
        if input == "exit" || input == "quit" {
            println!("Goodbye!");
            break;
        }
        
        // Process command
        match process_command(input) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}

fn process_command(input: &str) -> Result<()> {
    // Check for piping first
    if input.contains('|') {
        return process_pipe(input);
    }
    
    // Check for redirection
    let (cmd, redirect) = parse_redirection(input);
    
    // Execute command and capture output if needed
    let output = execute_single_command(&cmd)?;
    
    // Handle redirection
    if let Some((file, append)) = redirect {
        write_to_file(&output, &file, append)?;
    } else {
        print!("{}", output);
    }
    
    Ok(())
}

fn parse_redirection(input: &str) -> (String, Option<(String, bool)>) {
    if let Some(pos) = input.find(">>") {
        let (cmd, rest) = input.split_at(pos);
        let file = rest[2..].trim().to_string();
        (cmd.trim().to_string(), Some((file, true)))
    } else if let Some(pos) = input.find('>') {
        let (cmd, rest) = input.split_at(pos);
        let file = rest[1..].trim().to_string();
        (cmd.trim().to_string(), Some((file, false)))
    } else {
        (input.to_string(), None)
    }
}

fn write_to_file(content: &str, filename: &str, append: bool) -> Result<()> {
    let mut file = if append {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)?
    } else {
        File::create(filename)?
    };
    
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn process_pipe(input: &str) -> Result<()> {
    let commands: Vec<&str> = input.split('|').map(|s| s.trim()).collect();
    
    if commands.len() < 2 {
        anyhow::bail!("Invalid pipe syntax");
    }
    
    let mut output = execute_single_command(commands[0])?;
    
    for cmd in &commands[1..] {
        output = execute_with_input(cmd, &output)?;
    }
    
    print!("{}", output);
    Ok(())
}

fn execute_with_input(cmd: &str, input: &str) -> Result<String> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        anyhow::bail!("Empty command");
    }
    
    // For built-in commands that accept input
    match parts[0] {
        "cat" if parts.len() == 1 => Ok(input.to_string()),
        _ => execute_single_command(cmd),
    }
}

fn execute_single_command(input: &str) -> Result<String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.is_empty() {
        return Ok(String::new());
    }
    
    let command = parts[0];
    let args = &parts[1..];
    
    match command {
        "help" => help_command(),
        "pwd" => pwd_command(),
        "cd" => cd_command(args),
        "ls" => ls_command(args),
        "cat" => cat_command(args),
        "echo" => echo_command(args),
        "mkdir" => mkdir_command(args),
        "rmdir" => rmdir_command(args),
        "touch" => touch_command(args),
        "rm" => rm_command(args),
        "mv" => mv_command(args),
        _ => Err(anyhow::anyhow!("Command not found: {}", command)),
    }
}

