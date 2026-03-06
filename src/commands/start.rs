//! The `enx start` command — start the project.

use super::run::{collect_env_vars, execute_command};
use crate::config;

pub fn run() -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    if let Ok(project_config) = config::project::ProjectConfig::load_from_file(&current_dir) {
        let env = collect_env_vars(&project_config);
        if let Some(start_config) = project_config.start {
            for command in start_config.commands {
                execute_command(&command, &current_dir, &env)?;
            }

            return Ok(());
        }

        anyhow::bail!(
            "Could not parse start configuration. Please check the enx.toml file to ensure it is not corrupted."
        );
    }

    anyhow::bail!(
        "No project configuration found in the current directory. Are you sure you're in the right project?"
    )
}
