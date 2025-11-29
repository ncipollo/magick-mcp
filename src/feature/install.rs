use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Type of client to install for
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientType {
    Cursor,
    Claude,
    Both,
}

/// Paths to configuration files
#[derive(Debug, Clone)]
pub struct ConfigPaths {
    pub cursor_path: PathBuf,
    pub claude_path: PathBuf,
}

impl ConfigPaths {
    /// Get default configuration paths based on home directory
    pub fn from_home_dir() -> Result<Self, InstallError> {
        let home_dir = dirs::home_dir().ok_or(InstallError::HomeDirNotFound)?;

        Ok(ConfigPaths {
            cursor_path: home_dir.join(".cursor").join("mcp.json"),
            claude_path: home_dir.join(".claude.json"),
        })
    }
}

/// Installer for MCP configuration
pub struct MCPInstaller {
    client_type: ClientType,
    config_paths: ConfigPaths,
}

#[derive(Error, Debug)]
pub enum InstallError {
    #[error("Home directory not found")]
    HomeDirNotFound,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Failed to get executable path: {0}")]
    ExePathError(String),
    #[error("Invalid config: {0}")]
    InvalidConfig(String),
}

impl MCPInstaller {
    /// Create a new installer with the specified client type and config paths
    pub fn new(client_type: ClientType, config_paths: ConfigPaths) -> Self {
        MCPInstaller {
            client_type,
            config_paths,
        }
    }

    /// Install magick-mcp to the specified client(s)
    pub fn install(&self) -> Result<(), InstallError> {
        match self.client_type {
            ClientType::Cursor => {
                self.update_config(&self.config_paths.cursor_path)?;
            }
            ClientType::Claude => {
                self.update_config(&self.config_paths.claude_path)?;
            }
            ClientType::Both => {
                self.update_config(&self.config_paths.cursor_path)?;
                self.update_config(&self.config_paths.claude_path)?;
            }
        }
        Ok(())
    }

    /// Update a single configuration file
    fn update_config(&self, path: &Path) -> Result<(), InstallError> {
        // Get the path to the magick-mcp executable
        let exe_path =
            std::env::current_exe().map_err(|e| InstallError::ExePathError(e.to_string()))?;

        // Read existing config or create new one
        let mut config: Value = if path.exists() {
            let contents = fs::read_to_string(path)?;
            if contents.trim().is_empty() {
                json!({})
            } else {
                serde_json::from_str(&contents)?
            }
        } else {
            json!({})
        };

        // Ensure mcpServers object exists
        if config.get("mcpServers").is_none() {
            config["mcpServers"] = json!({});
        }

        // Get or create mcpServers object
        let mcp_servers = config
            .get_mut("mcpServers")
            .and_then(|v| v.as_object_mut())
            .ok_or_else(|| {
                InstallError::InvalidConfig("mcpServers is not an object".to_string())
            })?;

        // Add or update magick-mcp server entry
        mcp_servers.insert(
            "magick-mcp".to_string(),
            json!({
                "command": exe_path.to_string_lossy().to_string(),
                "args": ["mcp"]
            }),
        );

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write updated config back to file
        let pretty_json = serde_json::to_string_pretty(&config)?;
        fs::write(path, pretty_json)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_temp_config(dir: &TempDir, filename: &str, content: &str) -> PathBuf {
        let path = dir.path().join(filename);
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_install_cursor_only() {
        let temp_dir = TempDir::new().unwrap();
        let cursor_path = create_temp_config(
            &temp_dir,
            "mcp.json",
            r#"{"mcpServers": {"existing-server": {"command": "existing"}}}"#,
        );
        let claude_path = temp_dir.path().join("claude.json");

        let config_paths = ConfigPaths {
            cursor_path: cursor_path.clone(),
            claude_path,
        };

        let installer = MCPInstaller::new(ClientType::Cursor, config_paths);
        installer.install().unwrap();

        let contents = fs::read_to_string(&cursor_path).unwrap();
        let config: Value = serde_json::from_str(&contents).unwrap();

        assert!(config["mcpServers"]["magick-mcp"].is_object());
        assert!(config["mcpServers"]["existing-server"].is_object());
        assert_eq!(config["mcpServers"]["magick-mcp"]["args"], json!(["mcp"]));
    }

    #[test]
    fn test_install_claude_only() {
        let temp_dir = TempDir::new().unwrap();
        let cursor_path = temp_dir.path().join("mcp.json");
        let claude_path = create_temp_config(
            &temp_dir,
            "claude.json",
            r#"{"mcpServers": {"existing-server": {"command": "existing"}}}"#,
        );

        let config_paths = ConfigPaths {
            cursor_path,
            claude_path: claude_path.clone(),
        };

        let installer = MCPInstaller::new(ClientType::Claude, config_paths);
        installer.install().unwrap();

        let contents = fs::read_to_string(&claude_path).unwrap();
        let config: Value = serde_json::from_str(&contents).unwrap();

        assert!(config["mcpServers"]["magick-mcp"].is_object());
        assert!(config["mcpServers"]["existing-server"].is_object());
    }

    #[test]
    fn test_install_both() {
        let temp_dir = TempDir::new().unwrap();
        let cursor_path = create_temp_config(
            &temp_dir,
            "mcp.json",
            r#"{"mcpServers": {"cursor-server": {"command": "cursor"}}}"#,
        );
        let claude_path = create_temp_config(
            &temp_dir,
            "claude.json",
            r#"{"mcpServers": {"claude-server": {"command": "claude"}}}"#,
        );

        let config_paths = ConfigPaths {
            cursor_path: cursor_path.clone(),
            claude_path: claude_path.clone(),
        };

        let installer = MCPInstaller::new(ClientType::Both, config_paths);
        installer.install().unwrap();

        // Check cursor config
        let cursor_contents = fs::read_to_string(&cursor_path).unwrap();
        let cursor_config: Value = serde_json::from_str(&cursor_contents).unwrap();
        assert!(cursor_config["mcpServers"]["magick-mcp"].is_object());
        assert!(cursor_config["mcpServers"]["cursor-server"].is_object());

        // Check claude config
        let claude_contents = fs::read_to_string(&claude_path).unwrap();
        let claude_config: Value = serde_json::from_str(&claude_contents).unwrap();
        assert!(claude_config["mcpServers"]["magick-mcp"].is_object());
        assert!(claude_config["mcpServers"]["claude-server"].is_object());
    }

    #[test]
    fn test_preserve_existing_servers() {
        let temp_dir = TempDir::new().unwrap();
        let cursor_path = create_temp_config(
            &temp_dir,
            "mcp.json",
            r#"{
                "mcpServers": {
                    "server1": {"command": "cmd1", "args": ["arg1"]},
                    "server2": {"command": "cmd2"}
                }
            }"#,
        );
        let claude_path = temp_dir.path().join("claude.json");

        let config_paths = ConfigPaths {
            cursor_path: cursor_path.clone(),
            claude_path,
        };

        let installer = MCPInstaller::new(ClientType::Cursor, config_paths);
        installer.install().unwrap();

        let contents = fs::read_to_string(&cursor_path).unwrap();
        let config: Value = serde_json::from_str(&contents).unwrap();

        assert!(config["mcpServers"]["magick-mcp"].is_object());
        assert!(config["mcpServers"]["server1"].is_object());
        assert!(config["mcpServers"]["server2"].is_object());
        assert_eq!(config["mcpServers"]["server1"]["args"], json!(["arg1"]));
    }

    #[test]
    fn test_create_config_when_not_exists() {
        let temp_dir = TempDir::new().unwrap();
        let cursor_path = temp_dir.path().join("mcp.json");
        let claude_path = temp_dir.path().join("claude.json");

        let config_paths = ConfigPaths {
            cursor_path: cursor_path.clone(),
            claude_path,
        };

        let installer = MCPInstaller::new(ClientType::Cursor, config_paths);
        installer.install().unwrap();

        assert!(cursor_path.exists());
        let contents = fs::read_to_string(&cursor_path).unwrap();
        let config: Value = serde_json::from_str(&contents).unwrap();

        assert!(config["mcpServers"]["magick-mcp"].is_object());
    }

    #[test]
    fn test_update_existing_magick_mcp() {
        let temp_dir = TempDir::new().unwrap();
        let cursor_path = create_temp_config(
            &temp_dir,
            "mcp.json",
            r#"{
                "mcpServers": {
                    "magick-mcp": {"command": "old-path", "args": ["old"]}
                }
            }"#,
        );
        let claude_path = temp_dir.path().join("claude.json");

        let config_paths = ConfigPaths {
            cursor_path: cursor_path.clone(),
            claude_path,
        };

        let installer = MCPInstaller::new(ClientType::Cursor, config_paths);
        installer.install().unwrap();

        let contents = fs::read_to_string(&cursor_path).unwrap();
        let config: Value = serde_json::from_str(&contents).unwrap();

        assert_eq!(config["mcpServers"]["magick-mcp"]["args"], json!(["mcp"]));
        // Command should be updated to current exe path
        assert!(
            config["mcpServers"]["magick-mcp"]["command"]
                .as_str()
                .unwrap()
                .contains("magick-mcp")
        );
    }
}
