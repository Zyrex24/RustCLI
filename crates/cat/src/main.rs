use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, BufRead, Write};

#[derive(Parser, Debug)]
#[command(name = "cat")]
#[command(about = "Concatenate files and print to stdout", long_about = None)]
#[command(version)]
struct Args {
    /// Files to concatenate (use '-' for stdin)
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Number all output lines
    #[arg(short = 'n', long = "number")]
    number_lines: bool,

    /// Number non-empty output lines only
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank: bool,

    /// Show all characters (equivalent to -vET)
    #[arg(short = 'A', long = "show-all")]
    show_all: bool,

    /// Squeeze multiple adjacent blank lines into one
    #[arg(short = 's', long = "squeeze-blank")]
    squeeze_blank: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // -b overrides -n
    let number_mode = if args.number_nonblank {
        NumberMode::NonBlank
    } else if args.number_lines {
        NumberMode::All
    } else {
        NumberMode::None
    };
    
    let mut processor = LineProcessor::new(number_mode, args.show_all, args.squeeze_blank);
    
    for file in &args.files {
        process_file(file, &mut processor)
            .with_context(|| format!("Failed to process file: {}", file))?;
    }
    
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum NumberMode {
    None,
    All,
    NonBlank,
}

struct LineProcessor {
    number_mode: NumberMode,
    show_all: bool,
    squeeze_blank: bool,
    line_number: usize,
    last_was_blank: bool,
}

impl LineProcessor {
    fn new(number_mode: NumberMode, show_all: bool, squeeze_blank: bool) -> Self {
        Self {
            number_mode,
            show_all,
            squeeze_blank,
            line_number: 0,
            last_was_blank: false,
        }
    }
    
    fn process_line(&mut self, line: &[u8], stdout: &mut impl Write) -> io::Result<()> {
        let is_blank = line.is_empty() || (line.len() == 1 && line[0] == b'\n');
        
        // Handle squeeze blank
        if self.squeeze_blank && is_blank {
            if self.last_was_blank {
                return Ok(());
            }
            self.last_was_blank = true;
        } else {
            self.last_was_blank = false;
        }
        
        // Handle line numbering
        match self.number_mode {
            NumberMode::All => {
                self.line_number += 1;
                write!(stdout, "{:6}\t", self.line_number)?;
            }
            NumberMode::NonBlank => {
                if !is_blank {
                    self.line_number += 1;
                    write!(stdout, "{:6}\t", self.line_number)?;
                } else {
                    write!(stdout, "      \t")?;
                }
            }
            NumberMode::None => {}
        }
        
        // Process and write the line
        if self.show_all {
            self.write_with_show_all(line, stdout)?;
        } else {
            stdout.write_all(line)?;
        }
        
        stdout.write_all(b"\n")?;
        
        Ok(())
    }
    
    fn write_with_show_all(&self, line: &[u8], stdout: &mut impl Write) -> io::Result<()> {
        for &byte in line {
            match byte {
                b'\t' => write!(stdout, "^I")?,
                b'\n' => write!(stdout, "$")?,
                0..=31 => write!(stdout, "^{}", (byte + 64) as char)?,
                127 => write!(stdout, "^?")?,
                128..=255 => write!(stdout, "M-{}", if byte >= 128 + 32 && byte < 127 + 128 {
                    (byte - 128) as char
                } else {
                    '?'
                })?,
                _ => stdout.write_all(&[byte])?,
            }
        }
        Ok(())
    }
}

fn process_file(filename: &str, processor: &mut LineProcessor) -> Result<()> {
    let reader = common::io::open_input(filename)?;
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    
    for line_result in reader.split(b'\n') {
        let line = line_result?;
        processor.process_line(&line, &mut stdout_lock)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_number_mode_all() {
        let mut processor = LineProcessor::new(NumberMode::All, false, false);
        let mut output = Vec::new();
        
        processor.process_line(b"first", &mut output).unwrap();
        processor.process_line(b"second", &mut output).unwrap();
        
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("     1\tfirst"));
        assert!(result.contains("     2\tsecond"));
    }

    #[test]
    fn test_number_mode_nonblank() {
        let mut processor = LineProcessor::new(NumberMode::NonBlank, false, false);
        let mut output = Vec::new();
        
        processor.process_line(b"first", &mut output).unwrap();
        processor.process_line(b"", &mut output).unwrap();
        processor.process_line(b"third", &mut output).unwrap();
        
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("     1\tfirst"));
        assert!(result.contains("      \t\n")); // blank line with no number
        assert!(result.contains("     2\tthird"));
    }

    #[test]
    fn test_squeeze_blank() {
        let mut processor = LineProcessor::new(NumberMode::None, false, true);
        let mut output = Vec::new();
        
        processor.process_line(b"first", &mut output).unwrap();
        processor.process_line(b"", &mut output).unwrap();
        processor.process_line(b"", &mut output).unwrap();
        processor.process_line(b"", &mut output).unwrap();
        processor.process_line(b"second", &mut output).unwrap();
        
        let result = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 3); // first, one blank, second
    }

    #[test]
    fn test_show_all_tab() {
        let processor = LineProcessor::new(NumberMode::None, true, false);
        let mut output = Vec::new();
        
        processor.write_with_show_all(b"hello\tworld", &mut output).unwrap();
        
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("^I"));
    }
}

