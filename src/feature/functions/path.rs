use std::path::PathBuf;

/// Get the platform-specific directory for storing magick functions
///
/// Returns:
/// - Linux: `~/.local/share/magick-mcp/functions`
/// - macOS: `~/Library/Application Support/magick-mcp/functions`
/// - Windows: `C:\Users\<user>\AppData\Roaming\magick-mcp\functions`
///
/// Returns `None` if the data directory cannot be determined.
pub fn functions_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|dir| dir.join("magick-mcp").join("functions"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions_dir_returns_some() {
        let dir = functions_dir();
        assert!(dir.is_some());
        let path = dir.unwrap();
        assert!(path.to_string_lossy().contains("magick-mcp"));
        assert!(path.to_string_lossy().contains("functions"));
    }
}
