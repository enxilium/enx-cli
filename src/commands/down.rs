//! The `enx down` command — tear down the project environment.

use crate::config;
use super::run::{collect_env_vars, execute_command};

pub fn run() -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    if let Ok(project_config) = config::project::ProjectConfig::load_from_file(&current_dir) {
        let env = collect_env_vars(&project_config);
        if let Some(down_config) = project_config.down {
            let steps = if cfg!(target_os = "linux") {
                down_config.linux.as_ref().map(|p| &p.steps)
            } else if cfg!(target_os = "macos") {
                down_config.macos.as_ref().map(|p| &p.steps)
            } else if cfg!(target_os = "windows") {
                down_config.windows.as_ref().map(|p| &p.steps)
            } else {
                None
            }.unwrap_or(&down_config.steps);

            for command in steps {
                execute_command(&command, &current_dir, &env)?;
            }

            return Ok(());
        } 

        anyhow::bail!("Could not parse down configuration. Please check the enx.toml file to ensure it is not corrupted.");
    }

    anyhow::bail!("No project configuration found in the current directory. Are you sure you're in the right project?")
}
