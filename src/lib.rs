mod check;
pub mod cli;
mod install;
mod mcp;
mod shell;
mod which;

use check::MagickChecker;
use install::InstallError;
use install::MCPInstaller;
use shell::DefaultCommandRunner;
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
