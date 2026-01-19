//! Application configuration with file persistence.
//!
//! Config is stored in the platform-appropriate config directory:
//! - Linux: ~/.config/{{project-name}}/config.json
//! - macOS: ~/Library/Application Support/com.example.{{project-name}}/config.json
//! - Windows: %APPDATA%\example\{{project-name}}\config\config.json

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

/// Error type for configuration operations.
#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Json(serde_json::Error),
    NoConfigDir,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "IO error: {}", e),
            ConfigError::Json(e) => write!(f, "JSON error: {}", e),
            ConfigError::NoConfigDir => write!(f, "Could not determine config directory"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(e: serde_json::Error) -> Self {
        ConfigError::Json(e)
    }
}

/// Application configuration that persists to disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub username: String,

    #[serde(default)]
    pub dark_mode: bool,

    #[serde(default = "default_true")]
    pub notifications_enabled: bool,

    #[serde(default = "default_auto_save_interval")]
    pub auto_save_interval: u32,

    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_true() -> bool {
    true
}

fn default_auto_save_interval() -> u32 {
    300 // 5 minutes
}

fn default_theme() -> String {
    "system".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            username: String::new(),
            dark_mode: false,
            notifications_enabled: true,
            auto_save_interval: 300,
            theme: "system".to_string(),
        }
    }
}

impl AppConfig {
    /// Returns the configuration directory path for this application.
    pub fn config_dir() -> Option<PathBuf> {
        ProjectDirs::from("com", "example", "{{project-name}}")
            .map(|dirs| dirs.config_dir().to_path_buf())
    }

    /// Returns the full path to the config file.
    pub fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|dir| dir.join("config.json"))
    }

    /// Loads configuration from disk, returning defaults if file doesn't exist or is invalid.
    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::config_path().ok_or(ConfigError::NoConfigDir)?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path)?;

        // If JSON is invalid, return defaults rather than failing
        match serde_json::from_str(&contents) {
            Ok(config) => Ok(config),
            Err(_) => Ok(Self::default()),
        }
    }

    /// Saves configuration to disk, creating the config directory if needed.
    pub fn save(&self) -> Result<(), ConfigError> {
        let dir = Self::config_dir().ok_or(ConfigError::NoConfigDir)?;
        let path = dir.join("config.json");

        // Create config directory if it doesn't exist
        fs::create_dir_all(&dir)?;

        let json = serde_json::to_string_pretty(self)?;
        fs::write(&path, json)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert!(config.username.is_empty());
        assert!(!config.dark_mode);
        assert!(config.notifications_enabled);
        assert_eq!(config.auto_save_interval, 300);
        assert_eq!(config.theme, "system");
    }

    #[test]
    fn test_serialization() {
        let config = AppConfig {
            username: "testuser".to_string(),
            dark_mode: true,
            notifications_enabled: false,
            auto_save_interval: 60,
            theme: "dark".to_string(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: AppConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.username, "testuser");
        assert!(parsed.dark_mode);
        assert!(!parsed.notifications_enabled);
        assert_eq!(parsed.auto_save_interval, 60);
        assert_eq!(parsed.theme, "dark");
    }
}
