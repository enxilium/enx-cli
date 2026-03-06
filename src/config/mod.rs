pub mod global;
pub mod project;
pub mod registry;

use std::path::PathBuf;

pub fn config_dir() -> anyhow::Result<PathBuf> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    Ok(home_dir.join(".config").join("enx"))
}

pub fn registry_path() -> anyhow::Result<PathBuf> {
    Ok(config_dir()?.join("registry.toml"))
}

pub fn global_config_path() -> anyhow::Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}
