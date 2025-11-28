mod check;
mod shell;
mod which;

use check::MagickChecker;
use shell::DefaultCommandRunner;
use which::DefaultWhichChecker;

/// Check if ImageMagick is installed and return version or installation instructions
pub fn check() -> Result<String, String> {
    let which_checker = DefaultWhichChecker;
    let command_runner = DefaultCommandRunner;
    let checker = MagickChecker::new(&which_checker, &command_runner);
    checker.check_magick()
}
