//! The `enx start` command — start the project.

use super::run::{collect_env_vars, execute_command};
use crate::config;
use crate::output;

pub fn run() -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let project_config = config::project::ProjectConfig::load_from_file(&current_dir)?;
    let env = collect_env_vars(&project_config);
    let start_config = project_config
        .start
        .ok_or_else(|| anyhow::anyhow!("no [start] section found in enx.toml"))?;

    output::info(&format!("Starting {}...", project_config.project.name));
    for command in start_config.commands {
        execute_command(&command, &current_dir, &env)?;
    }

    Ok(())
}
