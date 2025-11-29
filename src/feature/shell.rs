use std::process::Command;
use thiserror::Error;

/// Error type for shell command execution failures
#[derive(Debug, Error)]
pub enum ShellError {
    #[error("Command execution failed: {message}\nCommand: {command} {args}")]
    ExecutionFailed {
        message: String,
        command: String,
        args: String,
    },
    #[error("Command output is not valid UTF-8\nCommand: {command} {args}")]
    InvalidUtf8 { command: String, args: String },
    #[error(
        "Command returned non-zero exit code (exit code: {exit_code})\nCommand: {command} {args}\nstdout: {stdout}\nstderr: {stderr}"
    )]
    NonZeroExit {
        exit_code: i32,
        command: String,
        args: String,
        stdout: String,
        stderr: String,
    },
    #[error("Missing required input variable: command contains $input but no input was provided")]
    MissingInputVariable,
}

/// Trait for executing shell commands in a mockable way
pub trait CommandRunner {
    /// Execute a command with the given arguments and return its output
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute
    /// * `args` - Arguments to pass to the command
    /// * `working_dir` - Optional working directory to set for the command
    fn execute(
        &self,
        command: &str,
        args: &[&str],
        working_dir: Option<&std::path::Path>,
    ) -> Result<String, ShellError>;
}

/// Default implementation of CommandRunner using std::process::Command
pub struct DefaultCommandRunner;

impl CommandRunner for DefaultCommandRunner {
    fn execute(
        &self,
        command: &str,
        args: &[&str],
        working_dir: Option<&std::path::Path>,
    ) -> Result<String, ShellError> {
        let path = std::env::var("PATH").ok();
        let mut cmd = Command::new(command);
        cmd.args(args).env_clear();
        if let Some(ref path_val) = path {
            cmd.env("PATH", path_val);
        }
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }
        let args_str = args.join(" ");
        let output = cmd.output().map_err(|e| ShellError::ExecutionFailed {
            message: e.to_string(),
            command: command.to_string(),
            args: args_str.clone(),
        })?;

        if !output.status.success() {
            let exit_code = output.status.code().unwrap_or(-1);
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(ShellError::NonZeroExit {
                exit_code,
                command: command.to_string(),
                args: args_str,
                stdout,
                stderr,
            });
        }

        String::from_utf8(output.stdout).map_err(|_| ShellError::InvalidUtf8 {
            command: command.to_string(),
            args: args_str,
        })
    }
}
