use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

pub type Result<T> = std::result::Result<T, CommonError>;

