//! The `enx down` command — tear down the project environment.

use super::run::{collect_env_vars, execute_command_with_spinner};
use crate::config;
use crate::output;

pub fn run() -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let project_config = config::project::ProjectConfig::load_from_file(&current_dir)?;
    let env = collect_env_vars(&project_config);
    let down_config = project_config
        .down
        .ok_or_else(|| anyhow::anyhow!("no [down] section found in enx.toml"))?;

    output::info(&format!("Tearing down {}...", project_config.project.name));

    let steps = if cfg!(target_os = "linux") {
        down_config.linux.as_ref().map(|p| &p.steps)
    } else if cfg!(target_os = "macos") {
        down_config.macos.as_ref().map(|p| &p.steps)
    } else if cfg!(target_os = "windows") {
        down_config.windows.as_ref().map(|p| &p.steps)
    } else {
        None
    }
    .unwrap_or(&down_config.steps);

    for command in steps {
        execute_command_with_spinner(command, &current_dir, &env)?;
    }

    output::success("Tear-down complete!");
    Ok(())
}
