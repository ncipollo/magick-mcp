mod check;
pub mod cli;
mod install;
mod magick;
mod mcp;
mod shell;
mod which;

use check::MagickChecker;
use install::InstallError;
use install::MCPInstaller;
use magick::MagickRunner;
use shell::{CommandRunner, DefaultCommandRunner, ShellError};
use which::DefaultWhichChecker;

pub use install::{ClientType, ConfigPaths};

/// Check if ImageMagick is installed and return version or installation instructions
pub fn check() -> Result<String, String> {
    let which_checker = DefaultWhichChecker;
    let command_runner = DefaultCommandRunner;
    let checker = MagickChecker::new(&which_checker, &command_runner);
    checker.check_magick()
}

/// Install magick-mcp to MCP client configuration
pub fn install(client_type: ClientType, config_paths: ConfigPaths) -> Result<(), InstallError> {
    let installer = MCPInstaller::new(client_type, config_paths);
    installer.install()
}

/// Execute an ImageMagick command
///
/// # Arguments
///
/// * `command` - A string containing ImageMagick command arguments, e.g., "test.png -negate test_negate.png"
/// * `workspace` - Optional workspace path to set as the working directory for the command
///
/// # Returns
///
/// Returns the command output as a String, or a ShellError if execution fails
pub fn magick(command: &str, workspace: Option<&std::path::Path>) -> Result<String, ShellError> {
    let command_runner = DefaultCommandRunner;
    let runner = MagickRunner::new(&command_runner, workspace);
    runner.execute(command)
}

/// Get ImageMagick help documentation
///
/// # Returns
///
/// Returns the help output from `magick --help` as a String, or a ShellError if execution fails
pub fn help() -> Result<String, ShellError> {
    let command_runner = DefaultCommandRunner;
    CommandRunner::execute(&command_runner, "magick", &["--help"], None)
}
