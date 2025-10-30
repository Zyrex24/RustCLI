use colored::*;

/// Wrapper for colored output that can be easily disabled.
pub struct ColorConfig {
    enabled: bool,
}

impl ColorConfig {
    pub fn new(enabled: bool) -> Self {
        if !enabled {
            colored::control::set_override(false);
        }
        Self { enabled }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self::new(true)
    }
}

/// Common color schemes for different file types.
pub mod schemes {
    use colored::*;

    pub fn directory(s: &str) -> ColoredString {
        s.blue().bold()
    }

    pub fn executable(s: &str) -> ColoredString {
        s.green().bold()
    }

    pub fn symlink(s: &str) -> ColoredString {
        s.cyan()
    }

    pub fn error(s: &str) -> ColoredString {
        s.red().bold()
    }

    pub fn warning(s: &str) -> ColoredString {
        s.yellow()
    }

    pub fn success(s: &str) -> ColoredString {
        s.green()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_config() {
        let config = ColorConfig::new(true);
        assert!(config.is_enabled());

        let config = ColorConfig::new(false);
        assert!(!config.is_enabled());
    }
}

