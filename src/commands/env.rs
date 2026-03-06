//! The `enx env` command — switch the active environment.
use crate::config;
use crate::output;
use crate::shell::Finalizer;

pub fn run(query: Option<&str>) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let project_config = config::project::ProjectConfig::load_from_file(&current_dir)
        .map_err(|_| anyhow::anyhow!("No project configuration found in the current directory. Are you sure you're in the right project?"))?;

    let env_config = project_config.env
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No environment configuration found in enx.toml. Please add an [env] section to define your environments."))?;

    match query {
        None => {
            // List available environments
            let names = env_config.environment_names();
            if names.is_empty() {
                output::info("No environments are currently defined in the configuration.");
            } else {
                output::info(&format!("Available environments: {}", names.join(", ")));
            }
            Ok(())
        }
        Some(env_name) => {
            // Switch to the specified environment
            let env = env_config.get_env_vars(env_name)?;

            let mut finalizers: Vec<Finalizer> = env
                .iter()
                .map(|(key, value)| Finalizer::SetEnv {
                    key: key.clone(),
                    value: value.clone(),
                })
                .collect();

            finalizers.push(Finalizer::SetEnv {
                key: "ENX_ENV".into(),
                value: env_name.into(),
            });

            crate::shell::write_finalizers(&finalizers)?;

            output::info(&format!("Switched to environment '{}'", env_name));

            for (key, value) in env {
                let display_value = if value.len() > 50 {
                    format!("{}...", &value[..47])
                } else {
                    value.clone()
                };

                output::detail(&format!("{}: {}", key, display_value));
            }

            Ok(())
        }
    }
}
