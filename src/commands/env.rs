//! The `enx env` command — switch the active environment.
use console::style;
use dialoguer::FuzzySelect;

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

    let names = env_config.environment_names();
    if names.is_empty() {
        output::info("No environments are currently defined in the configuration.");
        return Ok(());
    }

    // Determine which environment to activate.
    let env_name = match query {
        Some(name) => name.to_string(),
        None => {
            // No query — interactive picker
            let selection = FuzzySelect::with_theme(&output::theme())
                .with_prompt("Select environment")
                .items(&names)
                .default(0)
                .interact()?;

            names[selection].clone()
        }
    };

    // Switch to the specified environment
    let env = env_config.get_env_vars(&env_name)?;

    let mut finalizers: Vec<Finalizer> = env
        .iter()
        .map(|(key, value)| Finalizer::SetEnv {
            key: key.clone(),
            value: value.clone(),
        })
        .collect();

    finalizers.push(Finalizer::SetEnv {
        key: "ENX_ENV".into(),
        value: env_name.clone(),
    });

    crate::shell::write_finalizers(&finalizers)?;

    output::success(&format!(
        "Switched to environment '{}'",
        style(&env_name).color256(183).bold()
    ));

    let items: Vec<_> = env.iter().collect();
    let last = items.len().saturating_sub(1);

    for (i, (key, value)) in items.iter().enumerate() {
        let display_value = if value.len() > 50 {
            format!("{}...", &value[..47])
        } else {
            value.to_string()
        };

        let connector = if i == last { "└─" } else { "├─" };
        println!(
            "  {} {} {} {}",
            style(connector).color256(102),
            style(key).color256(116),
            style(":").color256(102),
            style(display_value).color256(102),
        );
    }

    Ok(())
}
