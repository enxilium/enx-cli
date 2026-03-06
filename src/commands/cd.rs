//! The `enx cd` command — match a project name and print its path.

use dialoguer::FuzzySelect;

use crate::config;
use crate::output;
use crate::shell::Finalizer;

pub fn run(name: &str) -> anyhow::Result<()> {
    let registry_path = config::registry_path()?;
    let registry = config::registry::Registry::load_from_file(&registry_path)?;

    if registry.projects.is_empty() {
        anyhow::bail!("no projects registered yet. Use `enx init` or `enx clone` to add one.");
    }

    if std::env::var("ENX_FINALIZER_FILE").is_err() {
        output::warning(
            "shell integration is not active. Make sure you have restarted your shell after running `enx setup`.",
        );
        return Ok(());
    }

    // Exact match — fast path
    if let Some(path) = registry.find_path_by_name(name) {
        if !path.exists() {
            anyhow::bail!(
                "project directory '{}' no longer exists. You may want to remove it from the registry with `enx remove`.",
                path.display()
            );
        }

        crate::shell::write_finalizers(&[Finalizer::Cd(path.clone())])?;
        return Ok(());
    }

    // Fuzzy match — collect names that contain the query as a substring,
    // then let the user pick interactively if there are multiple matches.
    let all_names: Vec<String> = registry.projects.keys().cloned().collect();
    let query_lower = name.to_lowercase();
    let matches: Vec<&String> = all_names
        .iter()
        .filter(|n| n.to_lowercase().contains(&query_lower))
        .collect();

    let chosen_name = match matches.len() {
        0 => {
            // No substring match — fall back to full FuzzySelect over all projects
            output::warning(&format!("no exact match for '{name}'. Pick a project:"));

            let selection = FuzzySelect::with_theme(&output::theme())
                .with_prompt("Project")
                .items(&all_names)
                .default(0)
                .interact()?;

            &all_names[selection]
        }
        1 => matches[0],
        _ => {
            // Multiple substring matches — let user pick
            let labels: Vec<&str> = matches.iter().map(|s| s.as_str()).collect();

            let selection = FuzzySelect::with_theme(&output::theme())
                .with_prompt("Multiple matches — pick a project")
                .items(&labels)
                .default(0)
                .interact()?;

            matches[selection]
        }
    };

    let path = registry.find_path_by_name(chosen_name).unwrap();

    if !path.exists() {
        anyhow::bail!(
            "project directory '{}' no longer exists. You may want to remove it from the registry with `enx remove`.",
            path.display()
        );
    }

    crate::shell::write_finalizers(&[Finalizer::Cd(path.clone())])?;

    Ok(())
}
