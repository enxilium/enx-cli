//! The `enx cd` command — match a project name and print its path.

use crate::config;
use crate::output;
use crate::shell::Finalizer;
use dialoguer::Confirm;

pub fn run(name: &str) -> anyhow::Result<()> {
    let registry_path = config::registry_path()?;
    let registry = config::registry::Registry::load_from_file(&registry_path)?;

    match registry.find_path_by_name(name) {
        Some(path) => {
            if !path.exists() {
                let should_delete = Confirm::new()
                    .with_prompt("Do you also want to delete the project directory?")
                    .default(false)
                    .interact()?;

                if should_delete {
                    std::fs::remove_dir_all(path)?;
                    output::success(&format!("Project directory '{}' deleted.", path.display()));
                } else {
                    output::warning(&format!(
                        "Project directory '{}' was not deleted. You may want to delete it manually if you no longer need it.",
                        path.display()
                    ));
                }

                return Ok(());
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
