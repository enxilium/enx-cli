//! The `enx init` command. Used to initialize a new project and add it to the registry.

use crate::config;
use crate::output;
use same_file::is_same_file;
use std::path::PathBuf;

/// Generate a default `enx.toml` with the project name filled in and all
/// sections present but empty.
fn default_enx_toml(project_name: &str) -> String {
    format!(
        r#"[project]
name = "{project_name}"

[env]

[up]
steps = [

]

[down]
steps = [

]

[start]
commands = [

]

[tasks]

[open]
"#
    )
}

/// Register a directory as an enx project.
///
/// If no path is given, uses the current directory. Otherwise, use the provided path.
/// The directory name is used as the project name.
/// A default `enx.toml` is created in the project root if one does not already exist.
pub fn run(path: Option<PathBuf>) -> anyhow::Result<()> {
    let project_path = match path {
        Some(p) => std::fs::canonicalize(p)?,
        None => std::env::current_dir()?,
    };

    let project_name = project_path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .ok_or_else(|| anyhow::anyhow!("failed to determine project name from path"))?
        .to_string();

    // --- registry --------------------------------------------------------
    let registry_path = config::registry_path()?;
    let mut registry = config::registry::Registry::load_from_file(&registry_path)?;

    if registry.find_path_by_name(&project_name).is_some() {
        return Err(anyhow::anyhow!(
            "a project with this name already exists in the registry"
        ));
    }

    for (name, path) in &registry.projects {
        if is_same_file(project_path.as_path(), &path)? {
            return Err(anyhow::anyhow!(
                "This directory is already registered as project '{}'",
                name
            ));
        }
    }

    registry.add_project(project_name.clone(), project_path.clone());
    registry.save_to_file(&registry_path)?;

    // --- enx.toml --------------------------------------------------------
    let toml_path = project_path.join("enx.toml");

    if toml_path.exists() {
        output::info(&format!(
            "enx.toml already exists at '{}'. Skipping creation.",
            toml_path.display()
        ));
    } else {
        std::fs::write(&toml_path, default_enx_toml(&project_name))?;
        output::success(&format!("Created enx.toml at '{}'", toml_path.display()));
    }

    output::success(&format!(
        "Project '{}' initialized at path '{}'",
        project_name,
        project_path.display()
    ));

    Ok(())
}
