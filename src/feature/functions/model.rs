use serde::{Deserialize, Serialize};

/// A function containing a series of ImageMagick commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    /// The name of the function
    pub name: String,
    /// Array of magick commands to execute in sequence
    pub commands: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_serialization() {
        let function = Function {
            name: "test_function".to_string(),
            commands: vec![
                "input.png -negate output1.png".to_string(),
                "output1.png -resize 50% output2.png".to_string(),
            ],
        };

        let json = serde_json::to_string(&function).unwrap();
        let deserialized: Function = serde_json::from_str(&json).unwrap();

        assert_eq!(function.name, deserialized.name);
        assert_eq!(function.commands, deserialized.commands);
    }
}
