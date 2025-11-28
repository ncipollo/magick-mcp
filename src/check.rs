use crate::shell::CommandRunner;
use crate::which::WhichChecker;

/// Checker for ImageMagick installation
pub struct MagickChecker<'a> {
    which_checker: &'a dyn WhichChecker,
    command_runner: &'a dyn CommandRunner,
}

impl<'a> MagickChecker<'a> {
    /// Create a new MagickChecker with the provided dependencies
    pub fn new(which_checker: &'a dyn WhichChecker, command_runner: &'a dyn CommandRunner) -> Self {
        MagickChecker {
            which_checker,
            command_runner,
        }
    }

    /// Check if ImageMagick is installed and return version or installation instructions
    pub fn check_magick(&self) -> Result<String, String> {
        match self.which_checker.find("magick") {
            Ok(_) => {
                // ImageMagick is installed, get version
                self.command_runner
                    .execute("magick", &["--version"])
                    .map_err(|e| format!("Failed to get ImageMagick version: {}", e))
            }
            Err(_) => {
                // ImageMagick is not installed, return platform-specific instructions
                Ok(self.get_installation_instructions())
            }
        }
    }

    /// Get platform-specific installation instructions
    fn get_installation_instructions(&self) -> String {
        let os = std::env::consts::OS;
        let instructions = match os {
            "macos" => "Install ImageMagick using Homebrew:\n  brew install imagemagick",
            "linux" => {
                "Install ImageMagick using your package manager:\n  sudo apt install imagemagick\n  or\n  sudo dnf install ImageMagick"
            }
            "windows" => {
                "Download and install ImageMagick from the official website.\n  Use winget: winget install ImageMagick.Q16-HDRI"
            }
            _ => "Install ImageMagick using your system's package manager.",
        };

        format!(
            "ImageMagick is not installed.\n\n{}\n\nFor more details, visit: https://imagemagick.org/script/download.php",
            instructions
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::{CommandRunner, ShellError};
    use crate::which::{WhichChecker, WhichError};
    use std::path::PathBuf;

    /// Mock implementation of WhichChecker for testing
    pub struct MockWhichChecker {
        pub found: bool,
    }

    impl WhichChecker for MockWhichChecker {
        fn find(&self, command: &str) -> Result<PathBuf, WhichError> {
            if self.found && command == "magick" {
                Ok(PathBuf::from("/usr/bin/magick"))
            } else {
                Err(WhichError::NotFound(command.to_string()))
            }
        }
    }

    /// Mock implementation of CommandRunner for testing
    pub struct MockCommandRunner {
        pub output: String,
        pub should_fail: bool,
    }

    impl CommandRunner for MockCommandRunner {
        fn execute(&self, _command: &str, _args: &[&str]) -> Result<String, ShellError> {
            if self.should_fail {
                Err(ShellError::NonZeroExit)
            } else {
                Ok(self.output.clone())
            }
        }
    }

    #[test]
    fn test_magick_checker_installed() {
        let which_checker = MockWhichChecker { found: true };
        let command_runner = MockCommandRunner {
            output: "Version: ImageMagick 7.1.2-8".to_string(),
            should_fail: false,
        };
        let checker = MagickChecker::new(&which_checker, &command_runner);
        let result = checker.check_magick();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Version: ImageMagick 7.1.2-8");
    }

    #[test]
    fn test_magick_checker_not_installed() {
        let which_checker = MockWhichChecker { found: false };
        let command_runner = MockCommandRunner {
            output: String::new(),
            should_fail: false,
        };
        let checker = MagickChecker::new(&which_checker, &command_runner);
        let result = checker.check_magick();
        assert!(result.is_ok());
        let instructions = result.unwrap();
        assert!(instructions.contains("ImageMagick is not installed"));
        assert!(instructions.contains("https://imagemagick.org/script/download.php"));
    }

    #[test]
    fn test_magick_checker_version_failure() {
        let which_checker = MockWhichChecker { found: true };
        let command_runner = MockCommandRunner {
            output: String::new(),
            should_fail: true,
        };
        let checker = MagickChecker::new(&which_checker, &command_runner);
        let result = checker.check_magick();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Failed to get ImageMagick version")
        );
    }

    #[test]
    fn test_platform_specific_instructions() {
        let which_checker = MockWhichChecker { found: false };
        let command_runner = MockCommandRunner {
            output: String::new(),
            should_fail: false,
        };
        let checker = MagickChecker::new(&which_checker, &command_runner);
        let result = checker.check_magick();
        assert!(result.is_ok());
        let instructions = result.unwrap();

        // Check that platform-specific content is included
        let os = std::env::consts::OS;
        match os {
            "macos" => assert!(instructions.contains("brew install")),
            "linux" => assert!(
                instructions.contains("apt install") || instructions.contains("dnf install")
            ),
            "windows" => {
                assert!(instructions.contains("winget") || instructions.contains("Download"))
            }
            _ => {} // Other platforms get generic message
        }
    }
}
