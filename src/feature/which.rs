use std::path::PathBuf;
use thiserror::Error;

/// Error type for which checker failures
#[derive(Debug, Error)]
pub enum WhichError {
    #[error("Command not found: {0}")]
    NotFound(String),
}

/// Trait for checking if a command exists in PATH (mockable wrapper around which crate)
pub trait WhichChecker {
    /// Check if a command exists in PATH
    fn find(&self, command: &str) -> Result<PathBuf, WhichError>;
}

/// Default implementation of WhichChecker using the which crate
pub struct DefaultWhichChecker;

impl WhichChecker for DefaultWhichChecker {
    fn find(&self, command: &str) -> Result<PathBuf, WhichError> {
        which::which(command).map_err(|_| WhichError::NotFound(command.to_string()))
    }
}
