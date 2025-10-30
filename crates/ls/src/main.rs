use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Parser, Debug)]
#[command(name = "ls")]
#[command(about = "List directory contents", long_about = None)]
#[command(version)]
struct Args {
    /// Directories or files to list
    #[arg(default_value = ".")]
    paths: Vec<String>,

    /// Use long listing format
    #[arg(short = 'l', long)]
    long: bool,

    /// Show hidden files (starting with .)
    #[arg(short = 'a', long = "all")]
    all: bool,

    /// Human-readable sizes (1K, 234M, 2G)
    #[arg(short = 'h', long = "human-readable")]
    human_readable: bool,

    /// Sort by modification time
    #[arg(short = 't', long)]
    time: bool,

    /// Reverse sort order
    #[arg(short = 'r', long = "reverse")]
    reverse: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    for path_str in &args.paths {
        list_path(path_str, &args)?;
    }
    
    Ok(())
}

fn list_path(path_str: &str, args: &Args) -> Result<()> {
    let path = Path::new(path_str);
    
    if !path.exists() {
        anyhow::bail!("cannot access '{}': No such file or directory", path_str);
    }
    
    if path.is_file() {
        let entry = FileEntry::from_path(path)?;
        print_entry(&entry, args);
    } else if path.is_dir() {
        list_directory(path, args)?;
    }
    
    Ok(())
}

fn list_directory(path: &Path, args: &Args) -> Result<()> {
    let mut entries = Vec::new();
    
    let dir_entries = fs::read_dir(path)
        .with_context(|| format!("Failed to read directory: {}", path.display()))?;
    
    for entry_result in dir_entries {
        let entry = entry_result?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        // Skip hidden files unless -a is specified
        if !args.all && file_name_str.starts_with('.') {
            continue;
        }
        
        let file_entry = FileEntry::from_dir_entry(&entry)?;
        entries.push(file_entry);
    }
    
    // Sort entries
    sort_entries(&mut entries, args);
    
    // Print entries
    for entry in entries {
        print_entry(&entry, args);
    }
    
    Ok(())
}

struct FileEntry {
    name: String,
    path: PathBuf,
    size: u64,
    modified: Option<SystemTime>,
    is_dir: bool,
    is_symlink: bool,
    #[cfg(unix)]
    permissions: u32,
}

impl FileEntry {
    fn from_path(path: &Path) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());
        
        Ok(Self {
            name,
            path: path.to_path_buf(),
            size: metadata.len(),
            modified: metadata.modified().ok(),
            is_dir: metadata.is_dir(),
            is_symlink: path.is_symlink(),
            #[cfg(unix)]
            permissions: metadata.permissions().mode(),
        })
    }
    
    fn from_dir_entry(entry: &fs::DirEntry) -> Result<Self> {
        let metadata = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().to_string();
        
        Ok(Self {
            name,
            path: entry.path(),
            size: metadata.len(),
            modified: metadata.modified().ok(),
            is_dir: metadata.is_dir(),
            is_symlink: entry.path().is_symlink(),
            #[cfg(unix)]
            permissions: metadata.permissions().mode(),
        })
    }
    
    #[cfg(unix)]
    fn permissions_string(&self) -> String {
        let mode = self.permissions;
        let file_type = if self.is_dir { 'd' } else if self.is_symlink { 'l' } else { '-' };
        
        format!(
            "{}{}{}{}{}{}{}{}{}{}",
            file_type,
            if mode & 0o400 != 0 { 'r' } else { '-' },
            if mode & 0o200 != 0 { 'w' } else { '-' },
            if mode & 0o100 != 0 { 'x' } else { '-' },
            if mode & 0o040 != 0 { 'r' } else { '-' },
            if mode & 0o020 != 0 { 'w' } else { '-' },
            if mode & 0o010 != 0 { 'x' } else { '-' },
            if mode & 0o004 != 0 { 'r' } else { '-' },
            if mode & 0o002 != 0 { 'w' } else { '-' },
            if mode & 0o001 != 0 { 'x' } else { '-' },
        )
    }
    
    #[cfg(not(unix))]
    fn permissions_string(&self) -> String {
        let file_type = if self.is_dir { 'd' } else if self.is_symlink { 'l' } else { '-' };
        format!("{}rw-rw-rw-", file_type)
    }
}

fn sort_entries(entries: &mut [FileEntry], args: &Args) {
    if args.time {
        entries.sort_by(|a, b| {
            let ord = b.modified.cmp(&a.modified); // newer first
            if args.reverse { ord.reverse() } else { ord }
        });
    } else {
        entries.sort_by(|a, b| {
            let ord = a.name.to_lowercase().cmp(&b.name.to_lowercase());
            if args.reverse { ord.reverse() } else { ord }
        });
    }
}

fn print_entry(entry: &FileEntry, args: &Args) {
    if args.long {
        print_long_format(entry, args);
    } else {
        println!("{}", entry.name);
    }
}

fn print_long_format(entry: &FileEntry, args: &Args) {
    let permissions = entry.permissions_string();
    let size = if args.human_readable {
        format_size_human(entry.size)
    } else {
        entry.size.to_string()
    };
    
    let modified = entry.modified
        .and_then(|t| {
            t.duration_since(SystemTime::UNIX_EPOCH).ok()
        })
        .map(|d| {
            let secs = d.as_secs();
            format_timestamp(secs)
        })
        .unwrap_or_else(|| "Unknown".to_string());
    
    println!("{} {:>8} {} {}", permissions, size, modified, entry.name);
}

fn format_size_human(size: u64) -> String {
    const UNITS: &[&str] = &["B", "K", "M", "G", "T"];
    let mut size = size as f64;
    let mut unit_idx = 0;
    
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    
    if unit_idx == 0 {
        format!("{}{}", size as u64, UNITS[unit_idx])
    } else {
        format!("{:.1}{}", size, UNITS[unit_idx])
    }
}

fn format_timestamp(secs: u64) -> String {
    // Simple formatting: just show a basic representation
    // In a real implementation, you'd use chrono or time crate for proper formatting
    let days = secs / 86400;
    let epoch_days = 719_163; // Days from year 0 to Unix epoch (1970-01-01)
    let total_days = epoch_days + days;
    
    // Simple Gregorian calendar calculation
    let (year, month, day) = days_to_date(total_days);
    
    let remaining_secs = secs % 86400;
    let hours = remaining_secs / 3600;
    let minutes = (remaining_secs % 3600) / 60;
    
    format!("{:04}-{:02}-{:02} {:02}:{:02}", year, month, day, hours, minutes)
}

fn days_to_date(total_days: u64) -> (u64, u64, u64) {
    // Simplified Gregorian calendar calculation
    let mut year = total_days / 365;
    let mut day_of_year = total_days % 365;
    
    // Rough adjustment for leap years
    let leap_years = year / 4 - year / 100 + year / 400;
    if day_of_year < leap_years {
        year -= 1;
        day_of_year = 365 + day_of_year;
    } else {
        day_of_year -= leap_years;
    }
    
    // Month lengths (non-leap year)
    const MONTH_DAYS: &[u64] = &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    
    let mut month = 1;
    let mut remaining = day_of_year;
    
    for &days_in_month in MONTH_DAYS {
        if remaining < days_in_month {
            break;
        }
        remaining -= days_in_month;
        month += 1;
    }
    
    let day = remaining + 1;
    
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_human() {
        assert_eq!(format_size_human(0), "0B");
        assert_eq!(format_size_human(1023), "1023B");
        assert_eq!(format_size_human(1024), "1.0K");
        assert_eq!(format_size_human(1536), "1.5K");
        assert_eq!(format_size_human(1048576), "1.0M");
        assert_eq!(format_size_human(1073741824), "1.0G");
    }

    #[test]
    fn test_format_size_human_large() {
        let size = 2_500_000_000_u64; // ~2.3 GB
        let result = format_size_human(size);
        assert!(result.ends_with('G'));
    }
}

