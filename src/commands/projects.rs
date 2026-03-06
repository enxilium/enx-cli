//! The `enx projects` command.

use console::style;

use crate::config;
use crate::output;

pub fn run() -> anyhow::Result<()> {
    let registry_path = config::registry_path()?;
    let registry = config::registry::Registry::load_from_file(&registry_path)?;

    if registry.projects.is_empty() {
        output::warning(
            "No projects registered yet. Use enx init or enx clone to initialize a project.",
        );
        return Ok(());
    }

    output::header("Registered projects");

    let items: Vec<_> = registry.projects.iter().collect();
    let last = items.len().saturating_sub(1);

    for (i, (name, path)) in items.iter().enumerate() {
        let styled_name = style(name).color256(183).bold();
        let styled_path = style(path.display().to_string()).color256(102);

        let connector = if i == last { "└─" } else { "├─" };

        if path.exists() {
            println!(
                "  {} {} {}",
                style(connector).color256(102),
                styled_name,
                styled_path
            );
        } else {
            println!(
                "  {} {} {} {}",
                style(connector).color256(102),
                style(name).color256(216),
                styled_path,
                style("(missing)").color256(216)
            );
        }
    }

    Ok(())
}
