//! The `enx up` command — bootstrap the project environment.

use super::run::{collect_env_vars, execute_command_with_spinner};
use crate::config;
use crate::output;

pub fn run() -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let project_config = config::project::ProjectConfig::load_from_file(&current_dir)?;
    let env = collect_env_vars(&project_config);
    let up_config = project_config
        .up
        .ok_or_else(|| anyhow::anyhow!("no [up] section found in enx.toml"))?;

    output::info(&format!("Bootstrapping {}...", project_config.project.name));

    let steps = if cfg!(target_os = "linux") {
        up_config.linux.as_ref().map(|p| &p.steps)
    } else if cfg!(target_os = "macos") {
        up_config.macos.as_ref().map(|p| &p.steps)
    } else if cfg!(target_os = "windows") {
        up_config.windows.as_ref().map(|p| &p.steps)
    } else {
        None
    }
    .unwrap_or(&up_config.steps);

    for command in steps {
        execute_command_with_spinner(command, &current_dir, &env)?;
    }

    output::success("Environment ready!");
    Ok(())
}
