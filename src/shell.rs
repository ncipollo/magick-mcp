use std::process::Command;
use thiserror::Error;

/// Error type for shell command execution failures
#[derive(Debug, Error)]
pub enum ShellError {
    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Command output is not valid UTF-8")]
    InvalidUtf8,
    #[error(
        "Command returned non-zero exit code (exit code: {exit_code})\nstdout: {stdout}\nstderr: {stderr}"
    )]
    NonZeroExit {
        exit_code: i32,
        stdout: String,
        stderr: String,
    },
}

/// Trait for executing shell commands in a mockable way
pub trait CommandRunner {
    /// Execute a command with the given arguments and return its output
    fn execute(&self, command: &str, args: &[&str]) -> Result<String, ShellError>;
}

/// Default implementation of CommandRunner using std::process::Command
pub struct DefaultCommandRunner;

impl CommandRunner for DefaultCommandRunner {
    fn execute(&self, command: &str, args: &[&str]) -> Result<String, ShellError> {
        let path = std::env::var("PATH").ok();
        let mut cmd = Command::new(command);
        cmd.args(args).env_clear();
        if let Some(ref path_val) = path {
            cmd.env("PATH", path_val);
        }
        let output = cmd
            .output()
            .map_err(|e| ShellError::ExecutionFailed(e.to_string()))?;

        if !output.status.success() {
            let exit_code = output.status.code().unwrap_or(-1);
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(ShellError::NonZeroExit {
                exit_code,
                stdout,
                stderr,
            });
        }

        String::from_utf8(output.stdout).map_err(|_| ShellError::InvalidUtf8)
    }
}
