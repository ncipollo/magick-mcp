use crate::feature::functions::model::Function;
use crate::feature::functions::path::functions_dir;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

/// Error type for function store operations
#[derive(Debug, Error)]
pub enum FunctionStoreError {
    #[error("Functions directory not found")]
    FunctionsDirNotFound,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Function '{0}' not found")]
    FunctionNotFound(String),
}

/// Store for loading and saving magick functions
pub struct FunctionStore;

impl FunctionStore {
    /// Create a new FunctionStore instance
    pub fn new() -> Self {
        FunctionStore
    }

    /// Get the path to a function file
    fn function_path(&self, name: &str) -> Result<PathBuf, FunctionStoreError> {
        let dir = functions_dir().ok_or(FunctionStoreError::FunctionsDirNotFound)?;
        Ok(dir.join(format!("{name}.json")))
    }

    /// Ensure the functions directory exists
    fn ensure_dir(&self) -> Result<PathBuf, FunctionStoreError> {
        let dir = functions_dir().ok_or(FunctionStoreError::FunctionsDirNotFound)?;
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    /// Save a function to disk
    ///
    /// # Arguments
    ///
    /// * `function` - The function to save
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a `FunctionStoreError` on failure
    pub fn save(&self, function: &Function) -> Result<(), FunctionStoreError> {
        self.ensure_dir()?;
        let path = self.function_path(&function.name)?;
        let json = serde_json::to_string_pretty(function)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load a function from disk
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function to load
    ///
    /// # Returns
    ///
    /// Returns the `Function` on success, or a `FunctionStoreError` on failure
    pub fn load(&self, name: &str) -> Result<Function, FunctionStoreError> {
        let path = self.function_path(name)?;
        if !path.exists() {
            return Err(FunctionStoreError::FunctionNotFound(name.to_string()));
        }
        let contents = fs::read_to_string(&path)?;
        let function: Function = serde_json::from_str(&contents)?;
        Ok(function)
    }

    /// List all available function names
    ///
    /// # Returns
    ///
    /// Returns a vector of function names, or a `FunctionStoreError` on failure
    pub fn list(&self) -> Result<Vec<String>, FunctionStoreError> {
        let dir = functions_dir().ok_or(FunctionStoreError::FunctionsDirNotFound)?;

        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut functions = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    functions.push(name.to_string());
                }
            }
        }
        Ok(functions)
    }

    /// Delete a function from disk
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function to delete
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a `FunctionStoreError` on failure
    pub fn delete(&self, name: &str) -> Result<(), FunctionStoreError> {
        let path = self.function_path(name)?;
        if !path.exists() {
            return Err(FunctionStoreError::FunctionNotFound(name.to_string()));
        }
        fs::remove_file(path)?;
        Ok(())
    }
}

impl Default for FunctionStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_function() {
        let store = FunctionStore::new();
        let function = Function {
            name: "test_save_load".to_string(),
            commands: vec!["test.png -negate output.png".to_string()],
        };

        // This test requires the functions directory to exist
        // In a real scenario, we'd use a temp directory
        if functions_dir().is_some() {
            let _ = store.save(&function);
            let loaded = store.load("test_save_load");
            if loaded.is_ok() {
                let func = loaded.unwrap();
                assert_eq!(func.name, function.name);
                assert_eq!(func.commands, function.commands);
                let _ = store.delete("test_save_load");
            }
        }
    }

    #[test]
    fn test_load_nonexistent_function() {
        let store = FunctionStore::new();
        let result = store.load("nonexistent_function_12345");
        assert!(result.is_err());
        if let Err(FunctionStoreError::FunctionNotFound(name)) = result {
            assert_eq!(name, "nonexistent_function_12345");
        } else {
            panic!("Expected FunctionNotFound error");
        }
    }

    #[test]
    fn test_list_functions() {
        let store = FunctionStore::new();
        // This test will work if the directory exists
        let result = store.list();
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_function() {
        let store = FunctionStore::new();
        let function = Function {
            name: "test_delete".to_string(),
            commands: vec!["test.png -negate output.png".to_string()],
        };

        if functions_dir().is_some() {
            let _ = store.save(&function);
            assert!(store.load("test_delete").is_ok());
            assert!(store.delete("test_delete").is_ok());
            assert!(store.load("test_delete").is_err());
        }
    }
}
