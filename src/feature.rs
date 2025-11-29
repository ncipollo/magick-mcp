mod check;
mod install;
mod magick;
mod shell;
mod which;

pub use check::MagickChecker;
pub use install::{ClientType, ConfigPaths, InstallError, MCPInstaller};
pub(crate) use magick::MagickRunner;
pub use shell::{CommandRunner, DefaultCommandRunner, ShellError};
pub use which::DefaultWhichChecker;
