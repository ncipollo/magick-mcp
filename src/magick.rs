use crate::shell::{CommandRunner, ShellError};

/// Runner for executing ImageMagick commands
pub(crate) struct MagickRunner<'a> {
    command_runner: &'a dyn CommandRunner,
}

impl<'a> MagickRunner<'a> {
    /// Create a new MagickRunner with the provided CommandRunner
    pub fn new(command_runner: &'a dyn CommandRunner) -> Self {
        MagickRunner { command_runner }
    }

    /// Execute an ImageMagick command by parsing the command string
    ///
    /// # Arguments
    ///
    /// * `command` - A string containing ImageMagick command arguments, e.g., "test.png -negate test_negate.png"
    ///
    /// # Returns
    ///
    /// Returns the command output as a String, or a ShellError if execution fails
    pub fn execute(&self, command: &str) -> Result<String, ShellError> {
        let args: Vec<&str> = command.split_whitespace().collect();
        self.command_runner.execute("magick", &args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::{CommandRunner, ShellError};

    /// Mock implementation of CommandRunner for testing
    pub struct MockCommandRunner {
        pub output: String,
        pub should_fail: bool,
        pub captured_command: std::cell::RefCell<Option<String>>,
        pub captured_args: std::cell::RefCell<Vec<String>>,
    }

    impl MockCommandRunner {
        pub fn new(output: String, should_fail: bool) -> Self {
            MockCommandRunner {
                output,
                should_fail,
                captured_command: std::cell::RefCell::new(None),
                captured_args: std::cell::RefCell::new(Vec::new()),
            }
        }
    }

    impl CommandRunner for MockCommandRunner {
        fn execute(&self, command: &str, args: &[&str]) -> Result<String, ShellError> {
            *self.captured_command.borrow_mut() = Some(command.to_string());
            *self.captured_args.borrow_mut() = args.iter().map(|s| s.to_string()).collect();

            if self.should_fail {
                Err(ShellError::NonZeroExit {
                    exit_code: 1,
                    stdout: String::new(),
                    stderr: "Mock error".to_string(),
                })
            } else {
                Ok(self.output.clone())
            }
        }
    }

    #[test]
    fn test_negate_operation() {
        let mock_runner = MockCommandRunner::new("Success".to_string(), false);
        let magick_runner = MagickRunner::new(&mock_runner);
        let result = magick_runner.execute("test.png -negate test_negate.png");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
        assert_eq!(
            *mock_runner.captured_command.borrow(),
            Some("magick".to_string())
        );
        assert_eq!(
            *mock_runner.captured_args.borrow(),
            vec!["test.png", "-negate", "test_negate.png"]
        );
    }

    #[test]
    fn test_resize_operation() {
        let mock_runner = MockCommandRunner::new("Resized".to_string(), false);
        let magick_runner = MagickRunner::new(&mock_runner);
        let result = magick_runner.execute("test.png -resize 50% test_small.png");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Resized");
        assert_eq!(
            *mock_runner.captured_command.borrow(),
            Some("magick".to_string())
        );
        assert_eq!(
            *mock_runner.captured_args.borrow(),
            vec!["test.png", "-resize", "50%", "test_small.png"]
        );
    }

    #[test]
    fn test_format_conversion() {
        let mock_runner = MockCommandRunner::new("Converted".to_string(), false);
        let magick_runner = MagickRunner::new(&mock_runner);
        let result = magick_runner.execute("test.jpg -format png test.png");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Converted");
        assert_eq!(
            *mock_runner.captured_command.borrow(),
            Some("magick".to_string())
        );
        assert_eq!(
            *mock_runner.captured_args.borrow(),
            vec!["test.jpg", "-format", "png", "test.png"]
        );
    }

    #[test]
    fn test_multiple_operations() {
        let mock_runner = MockCommandRunner::new("Modified".to_string(), false);
        let magick_runner = MagickRunner::new(&mock_runner);
        let result = magick_runner.execute("test.png -rotate 90 -blur 5x2 test_modified.png");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Modified");
        assert_eq!(
            *mock_runner.captured_command.borrow(),
            Some("magick".to_string())
        );
        assert_eq!(
            *mock_runner.captured_args.borrow(),
            vec![
                "test.png",
                "-rotate",
                "90",
                "-blur",
                "5x2",
                "test_modified.png"
            ]
        );
    }
}
