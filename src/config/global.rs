//! Global configuration for enx.
//! Defines default settings and global tasks that can be run anywhere.

use crate::config::project::TaskConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalConfig {
    pub defaults: Option<DefaultsConfig>,
    pub tasks: Option<HashMap<String, TaskConfig>>,
    #[serde(default)]
    pub is_configured: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DefaultsConfig {
    pub projects_dir: Option<String>,
}

impl GlobalConfig {
    /// Load the global configuration from a given file path. Returns a GlobalConfig struct on success.
    pub fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            // If the config file doesn't exist, return an empty config
            return Ok(GlobalConfig {
                defaults: None,
                tasks: None,
                is_configured: false,
            });
        }

        let content = std::fs::read_to_string(path)?;
        let config: GlobalConfig = toml::from_str(&content)?;

        Ok(config)
    }

    pub fn projects_dir(&self) -> Option<String> {
        self.defaults.as_ref()?.projects_dir.clone()
    }

    pub fn is_setup_complete(&self) -> bool {
        self.is_configured
    }
}
