//! Project registry - list of all known projects and their paths.
//! This is used to quickly find a project by name and get its path.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    pub projects: HashMap<String, PathBuf>
}

impl Registry {
    /// Load the registry from a given file path. Returns a Registry struct on success.
    pub fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            // If the registry file doesn't exist, return an empty registry
            return Ok(Registry { projects: HashMap::new() });
        }

        let content = std::fs::read_to_string(path)?;
        let registry: Registry = toml::from_str(&content)?;

        Ok(registry)
    }

    /// Save the registry to a given file path.
    pub fn save_to_file(&self, path: &Path) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn find_path_by_name(&self, name: &str) -> Option<&PathBuf> {
        self.projects.get(name)
    }

    pub fn add_project(&mut self, name: String, path: PathBuf) {
        self.projects.insert(name, path);
    }

    /// We are guarnateed to have a project with this name (checked at the CLI layer), so we can just remove it without checking.
    pub fn remove_project(&mut self, name: &str) {
        self.projects.remove(name);
    }
}
