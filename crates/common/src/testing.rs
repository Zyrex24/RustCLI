use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Creates a temporary test file with the given content.
pub fn create_test_file<P: AsRef<Path>>(path: P, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Creates a temporary directory structure for testing.
pub fn create_test_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

/// Helper to create a test fixture directory with multiple files.
pub struct TestFixture {
    pub root: PathBuf,
}

impl TestFixture {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn create_file(&self, name: &str, content: &str) -> std::io::Result<PathBuf> {
        let path = self.root.join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        create_test_file(&path, content)?;
        Ok(path)
    }

    pub fn create_dir(&self, name: &str) -> std::io::Result<PathBuf> {
        let path = self.root.join(name);
        create_test_dir(&path)?;
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_create_test_file() {
        let temp_dir = env::temp_dir();
        let file_path = temp_dir.join("test_file.txt");
        create_test_file(&file_path, "test content").unwrap();
        assert!(file_path.exists());
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "test content");
        fs::remove_file(file_path).unwrap();
    }
}

