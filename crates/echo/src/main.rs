use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(name = "echo")]
#[command(about = "Display a line of text", long_about = None)]
#[command(version)]
struct Args {
    /// Suppress trailing newline
    #[arg(short = 'n', long)]
    no_newline: bool,

    /// Enable interpretation of backslash escapes
    #[arg(short = 'e', long)]
    escape: bool,

    /// Disable interpretation of backslash escapes (default)
    #[arg(short = 'E', long)]
    no_escape: bool,

    /// Text to echo
    #[arg(trailing_var_arg = true)]
    text: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let output = process_echo(&args)?;
    print_output(&output, args.no_newline)?;
    
    Ok(())
}

fn process_echo(args: &Args) -> Result<String> {
    let text = args.text.join(" ");
    
    // -E flag explicitly disables escape interpretation
    // Otherwise, -e flag enables it
    let should_interpret_escapes = !args.no_escape && args.escape;
    
    if should_interpret_escapes {
        Ok(interpret_escapes(&text))
    } else {
        Ok(text)
    }
}

fn interpret_escapes(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next) = chars.peek() {
                chars.next(); // consume the next character
                match next {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    'a' => result.push('\x07'), // alert (bell)
                    'b' => result.push('\x08'), // backspace
                    'f' => result.push('\x0C'), // form feed
                    'v' => result.push('\x0B'), // vertical tab
                    '0' => result.push('\0'),   // null
                    _ => {
                        // If not a recognized escape, keep the backslash and character
                        result.push('\\');
                        result.push(next);
                    }
                }
            } else {
                // Trailing backslash
                result.push('\\');
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}

fn print_output(output: &str, no_newline: bool) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    handle.write_all(output.as_bytes())?;
    
    if !no_newline {
        handle.write_all(b"\n")?;
    }
    
    handle.flush()?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret_escapes_newline() {
        assert_eq!(interpret_escapes("hello\\nworld"), "hello\nworld");
    }

    #[test]
    fn test_interpret_escapes_tab() {
        assert_eq!(interpret_escapes("hello\\tworld"), "hello\tworld");
    }

    #[test]
    fn test_interpret_escapes_backslash() {
        assert_eq!(interpret_escapes("hello\\\\world"), "hello\\world");
    }

    #[test]
    fn test_interpret_escapes_multiple() {
        assert_eq!(interpret_escapes("a\\nb\\tc\\rd"), "a\nb\tc\rd");
    }

    #[test]
    fn test_interpret_escapes_unknown() {
        assert_eq!(interpret_escapes("hello\\xworld"), "hello\\xworld");
    }

    #[test]
    fn test_interpret_escapes_trailing_backslash() {
        assert_eq!(interpret_escapes("hello\\"), "hello\\");
    }

    #[test]
    fn test_process_echo_no_escape() {
        let args = Args {
            no_newline: false,
            escape: false,
            no_escape: false,
            text: vec!["hello\\nworld".to_string()],
        };
        assert_eq!(process_echo(&args).unwrap(), "hello\\nworld");
    }

    #[test]
    fn test_process_echo_with_escape() {
        let args = Args {
            no_newline: false,
            escape: true,
            no_escape: false,
            text: vec!["hello\\nworld".to_string()],
        };
        assert_eq!(process_echo(&args).unwrap(), "hello\nworld");
    }

    #[test]
    fn test_process_echo_explicit_no_escape() {
        let args = Args {
            no_newline: false,
            escape: true,
            no_escape: true,
            text: vec!["hello\\nworld".to_string()],
        };
        assert_eq!(process_echo(&args).unwrap(), "hello\\nworld");
    }
}

