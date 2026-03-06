//! The `enx open` command — open a project-defined target.

use crate::config;
use super::run::{collect_env_vars, execute_command};

pub fn run(target: &str) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    if let Ok(project_config) = config::project::ProjectConfig::load_from_file(&current_dir) {
        let env = collect_env_vars(&project_config);
        if let Some(open_config) = project_config.open {
            for (config_target, command) in open_config {
                if config_target == target {
                    execute_command(&command, &current_dir, &env)?;
                    return Ok(());
                }
            }
            // If we reach this point, it means we didn't find a matching target in the configuration.
            anyhow::bail!("No open configuration found for target '{}'. Please check the enx.toml file to ensure it is defined correctly.", target);
        } 

        anyhow::bail!("Could not parse open configuration. Please check the enx.toml file to ensure it is not corrupted.");
    }

    anyhow::bail!("No project configuration found in the current directory. Are you sure you're in the right project?")
}
