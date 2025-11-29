use crate::feature::functions::model::Function;
use crate::feature::magick::MagickRunner;
use crate::feature::shell::{CommandRunner, ShellError};
use std::path::Path;

/// Runner for executing magick functions (sequences of commands)
pub struct FunctionRunner<'a> {
    magick_runner: MagickRunner<'a>,
}

impl<'a> FunctionRunner<'a> {
    /// Create a new FunctionRunner with the provided CommandRunner and optional workspace path
    ///
    /// # Arguments
    ///
    /// * `command_runner` - The CommandRunner to use for executing commands
    /// * `workspace` - Optional workspace path to set as the working directory
    pub fn new(command_runner: &'a dyn CommandRunner, workspace: Option<&'a Path>) -> Self {
        FunctionRunner {
            magick_runner: MagickRunner::new(command_runner, workspace),
        }
    }

    /// Execute all commands in a function sequentially
    ///
    /// # Arguments
    ///
    /// * `function` - The function containing commands to execute
    /// * `input` - Optional input value to replace `$input` placeholders in commands
    ///
    /// # Returns
    ///
    /// Returns a vector of command outputs, or the first `ShellError` encountered
    ///
    /// # Errors
    ///
    /// Returns `ShellError::MissingInputVariable` if a command contains `$input` but no input was provided
    pub fn run(&self, function: &Function, input: Option<&str>) -> Result<Vec<String>, ShellError> {
        let mut outputs = Vec::new();
        for command in &function.commands {
            // Check if command contains $input placeholder
            if command.contains("$input") {
                // If $input is found but no input provided, return error
                let input_value = input.ok_or(ShellError::MissingInputVariable)?;
                // Replace $input with the provided value
                let processed_command = command.replace("$input", input_value);
                let output = self.magick_runner.execute(&processed_command)?;
                outputs.push(output);
            } else {
                // No $input placeholder, execute command as-is
                let output = self.magick_runner.execute(command)?;
                outputs.push(output);
            }
        }
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::shell::{CommandRunner, ShellError};

    /// Mock implementation of CommandRunner for testing
    struct MockCommandRunner {
        output: String,
        should_fail: bool,
        call_count: std::cell::RefCell<usize>,
    }

    impl MockCommandRunner {
        fn new(output: String, should_fail: bool) -> Self {
            MockCommandRunner {
                output,
                should_fail,
                call_count: std::cell::RefCell::new(0),
            }
        }
    }

    impl CommandRunner for MockCommandRunner {
        fn execute(
            &self,
            _command: &str,
            _args: &[&str],
            _working_dir: Option<&std::path::Path>,
        ) -> Result<String, ShellError> {
            *self.call_count.borrow_mut() += 1;
            if self.should_fail {
                Err(ShellError::NonZeroExit {
                    exit_code: 1,
                    command: "magick".to_string(),
                    args: "test".to_string(),
                    stdout: String::new(),
                    stderr: "Mock error".to_string(),
                })
            } else {
                Ok(self.output.clone())
            }
        }
    }

    #[test]
    fn test_run_function_success() {
        let mock_runner = MockCommandRunner::new("Success".to_string(), false);
        let function_runner = FunctionRunner::new(&mock_runner, None);
        let function = Function {
            name: "test".to_string(),
            commands: vec![
                "input.png -negate output1.png".to_string(),
                "output1.png -resize 50% output2.png".to_string(),
            ],
        };

        let result = function_runner.run(&function, None);
        assert!(result.is_ok());
        let outputs = result.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Success");
        assert_eq!(outputs[1], "Success");
        assert_eq!(*mock_runner.call_count.borrow(), 2);
    }

    #[test]
    fn test_run_function_stops_on_error() {
        let failing_runner = MockCommandRunner::new("Error".to_string(), true);
        let function_runner = FunctionRunner::new(&failing_runner, None);
        let function = Function {
            name: "test".to_string(),
            commands: vec![
                "input.png -negate output1.png".to_string(),
                "output1.png -resize 50% output2.png".to_string(),
            ],
        };

        let result = function_runner.run(&function, None);
        assert!(result.is_err());
        assert_eq!(*failing_runner.call_count.borrow(), 1);
    }

    #[test]
    fn test_run_empty_function() {
        let mock_runner = MockCommandRunner::new("Success".to_string(), false);
        let function_runner = FunctionRunner::new(&mock_runner, None);
        let function = Function {
            name: "test".to_string(),
            commands: vec![],
        };

        let result = function_runner.run(&function, None);
        assert!(result.is_ok());
        let outputs = result.unwrap();
        assert_eq!(outputs.len(), 0);
        assert_eq!(*mock_runner.call_count.borrow(), 0);
    }

    #[test]
    fn test_run_function_with_input_substitution() {
        let mock_runner = MockCommandRunner::new("Success".to_string(), false);
        let function_runner = FunctionRunner::new(&mock_runner, None);
        let function = Function {
            name: "test".to_string(),
            commands: vec!["$input -negate output.png".to_string()],
        };

        let result = function_runner.run(&function, Some("photo.png"));
        assert!(result.is_ok());
        let outputs = result.unwrap();
        assert_eq!(outputs.len(), 1);
        assert_eq!(*mock_runner.call_count.borrow(), 1);
    }

    #[test]
    fn test_run_function_missing_input_variable() {
        let mock_runner = MockCommandRunner::new("Success".to_string(), false);
        let function_runner = FunctionRunner::new(&mock_runner, None);
        let function = Function {
            name: "test".to_string(),
            commands: vec!["$input -negate output.png".to_string()],
        };

        let result = function_runner.run(&function, None);
        assert!(result.is_err());
        if let Err(ShellError::MissingInputVariable) = result {
            // Expected error
        } else {
            panic!("Expected MissingInputVariable error");
        }
        // Should not execute any commands
        assert_eq!(*mock_runner.call_count.borrow(), 0);
    }
}
