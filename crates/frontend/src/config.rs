//! Frontend mirror of the backend AppConfig struct.
//!
//! This struct mirrors the backend configuration for serialization/deserialization
//! when communicating via IPC.

use serde::{Deserialize, Serialize};

/// Application configuration mirroring the backend struct.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    300
}

fn default_theme() -> String {
    "system".to_string()
}
