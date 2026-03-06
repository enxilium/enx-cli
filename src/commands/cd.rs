//! The `enx cd` command — match a project name and print its path.

use crate::config;
use crate::output;
use crate::shell::Finalizer;

pub fn run(name: &str) -> anyhow::Result<()> {
    let registry_path = config::registry_path()?;
    let registry = config::registry::Registry::load_from_file(&registry_path)?;

    match registry.find_path_by_name(name) {
        Some(path) => {
            if !path.exists() {
                anyhow::bail!(
                    "project directory '{}' no longer exists. You may want to remove it from the registry with `enx remove`.",
                    path.display()
                );
            }

            crate::shell::write_finalizers(&[Finalizer::Cd(path.clone())])?;

            Ok(())
        }
        None => {
            if !registry.projects.is_empty() {
                output::error(
                    "No project with this name found in the registry. Here are the registered projects:",
                );
                for (name, _) in registry.projects {
                    output::info(&name.to_string());
                }

                return Ok(());
            }

            Err(anyhow::anyhow!(
                "No project with this name found in the registry."
            ))
        }
    }
}
