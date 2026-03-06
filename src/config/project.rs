//! Per-project enx.toml cnofiguration schema.
//! This defines the structure for how enx.toml should look like and handles reading it.
use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub project: ProjectInfo,
    pub env: Option<EnvConfig>,
    pub up: Option<LifecycleConfig>,
    pub down: Option<LifecycleConfig>,
    pub start: Option<StartConfig>,
    pub tasks: Option<HashMap<String, TaskConfig>>,
    pub open: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    #[serde(flatten)]
    pub environments: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct LifecycleConfig {
    pub steps: Vec<String>,
    /// Optional platform-specific steps. If defined, takes precedence over the generic steps for that platform.
    pub linux: Option<PlatformSteps>,
    pub macos: Option<PlatformSteps>,
    pub windows: Option<PlatformSteps>,
}

#[derive(Debug, Deserialize)]
pub struct PlatformSteps {
    pub steps: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct StartConfig {
    pub commands: Vec<String>,
}

/// Custom tasks, can be run with `enx run <task-name>`.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskConfig {
    pub command: String,
    pub description: Option<String>,
}

impl ProjectConfig {
    /// Load the project configuration from a given file path. Returns a ProjectConfig struct on success.
    pub fn load_from_file(project_dir: &Path) -> anyhow::Result<Self> {
        let path = project_dir.join("enx.toml");
        let content = std::fs::read_to_string(&path)?;
        let config: ProjectConfig = toml::from_str(&content)?;

        Ok(config)
    }
}

impl EnvConfig {
    /// Load environment variables from the file mapped to `env_name`.
    pub fn get_env_vars(&self, env_name: &str) -> anyhow::Result<HashMap<String, String>> {
        let environments = self
            .environments
            .as_ref()
            .context("no environment file mappings defined under [env]")?;

        let env_file_path = environments
            .get(env_name)
            .with_context(|| format!("environment '{env_name}' is not defined under [env]"))?;

        // Read the dotenv file and parse key=value pairs without polluting
        // the current process environment.
        let content = std::fs::read_to_string(env_file_path)
            .with_context(|| format!("failed to read environment file '{env_file_path}'"))?;

        let mut vars = HashMap::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = trimmed.split_once('=') {
                let key = key.trim().to_string();
                // Strip optional surrounding quotes from the value
                let value = value.trim();
                let value = value
                    .strip_prefix('"')
                    .and_then(|v| v.strip_suffix('"'))
                    .or_else(|| value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')))
                    .unwrap_or(value)
                    .to_string();
                vars.insert(key, value);
            }
        }
        Ok(vars)
    }

    pub fn environment_names(&self) -> Vec<String> {
        self.environments
            .as_ref()
            .map(|envs| envs.keys().cloned().collect())
            .unwrap_or_default()
    }
}
