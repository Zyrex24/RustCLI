use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;

/// Creates a buffered reader for the given file path.
/// Returns a reader for stdin if the path is "-".
pub fn open_input(path: &str) -> io::Result<Box<dyn BufRead>> {
    if path == "-" {
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        let file = File::open(path)?;
        Ok(Box::new(BufReader::new(file)))
    }
}

/// Creates a buffered reader from a file.
pub fn buffered_reader<P: AsRef<Path>>(path: P) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

/// Creates a buffered writer to stdout.
pub fn stdout_writer() -> BufWriter<io::Stdout> {
    BufWriter::new(io::stdout())
}

/// Creates a buffered writer to stderr.
pub fn stderr_writer() -> BufWriter<io::Stderr> {
    BufWriter::new(io::stderr())
}

/// Reads all bytes from the given reader.
pub fn read_all_bytes<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Writes data to the given writer, flushing afterwards.
pub fn write_and_flush<W: Write>(mut writer: W, data: &[u8]) -> io::Result<()> {
    writer.write_all(data)?;
    writer.flush()?;
    Ok(())
}

/// Counts the number of lines in the given reader.
pub fn count_lines<R: BufRead>(reader: R) -> io::Result<usize> {
    Ok(reader.lines().count())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_all_bytes() {
        let data = b"Hello, World!";
        let cursor = Cursor::new(data);
        let result = read_all_bytes(cursor).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_count_lines() {
        let data = "line1\nline2\nline3\n";
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let count = count_lines(reader).unwrap();
        assert_eq!(count, 3);
    }
}

