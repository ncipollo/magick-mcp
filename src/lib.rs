pub mod cli;
mod feature;
mod mcp;

use feature::DefaultWhichChecker;
use feature::InstallError;
use feature::MCPInstaller;
use feature::MagickChecker;
use feature::{CommandRunner, DefaultCommandRunner, ShellError};
use feature::{Function, FunctionRunner, FunctionStore, FunctionStoreError};

pub use feature::{ClientType, ConfigPaths};

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
    let runner = feature::MagickRunner::new(&command_runner, workspace);
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

/// Save a magick function to disk
///
/// # Arguments
///
/// * `function` - The function to save
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `FunctionStoreError` on failure
pub fn save_function(function: Function) -> Result<(), FunctionStoreError> {
    let store = FunctionStore::new();
    store.save(&function)
}

/// Load a magick function from disk
///
/// # Arguments
///
/// * `name` - The name of the function to load
///
/// # Returns
///
/// Returns the `Function` on success, or a `FunctionStoreError` on failure
pub fn load_function(name: &str) -> Result<Function, FunctionStoreError> {
    let store = FunctionStore::new();
    store.load(name)
}

/// List all available magick function names
///
/// # Returns
///
/// Returns a vector of function names, or a `FunctionStoreError` on failure
pub fn list_functions() -> Result<Vec<String>, FunctionStoreError> {
    let store = FunctionStore::new();
    store.list()
}

/// Delete a magick function from disk
///
/// # Arguments
///
/// * `name` - The name of the function to delete
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `FunctionStoreError` on failure
pub fn delete_function(name: &str) -> Result<(), FunctionStoreError> {
    let store = FunctionStore::new();
    store.delete(name)
}

/// Execute a magick function (run all commands in sequence)
///
/// # Arguments
///
/// * `function` - The function containing commands to execute
/// * `workspace` - Optional workspace path to set as the working directory for commands
/// * `input` - Optional input value to replace `$input` placeholders in commands
///
/// # Returns
///
/// Returns a vector of command outputs, or the first `ShellError` encountered
///
/// # Errors
///
/// Returns `ShellError::MissingInputVariable` if a command contains `$input` but no input was provided
pub fn run_function(
    function: &Function,
    workspace: Option<&std::path::Path>,
    input: Option<&str>,
) -> Result<Vec<String>, ShellError> {
    let command_runner = DefaultCommandRunner;
    let runner = FunctionRunner::new(&command_runner, workspace);
    runner.run(function, input)
}
