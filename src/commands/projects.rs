//! The `enx projects` command.

use colored::Colorize;

use crate::config;
use crate::output;

pub fn run() -> anyhow::Result<()> {
    let registry_path = config::registry_path()?;
    let registry = config::registry::Registry::load_from_file(&registry_path)?;

    if registry.projects.is_empty() {
        output::warning("No projects registered yet. Use enx init or enx clone to initialize a project.");
        return Ok(());
    }

    for (name, path) in &registry.projects {
        if path.exists() {
            println!("  {}", name.blue());
        } else {
            println!("{} {} (path may not exist)", "⚠".yellow(), name.yellow());
        }
    }

    Ok(())
}