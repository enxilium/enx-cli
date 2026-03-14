//! The `enx init` command. Used to initialize a new project and add it to the registry.

use crate::config;
use crate::output;
use same_file::is_same_file;
use std::path::PathBuf;

/// Generate a default `enx.toml` with the project name filled in and
/// commented examples that reflect the exact expected schema.
fn default_enx_toml(project_name: &str) -> String {
    format!(
        r#"# enx project configuration
    # Example: name = "my-app"
    # Example: steps = ["cmd1", "cmd2"]
    # Example: [tasks.test]

[project]
# display name shown in `enx projects`
name = "{project_name}"

# [env] maps environment name -> dotenv file path.
# `enx env <name>` looks up the name here and loads that file.
[env]
# dev = ".env"
# staging = ".env.staging"

# [up] uses a list of shell commands run by `enx up`.
[up]
steps = [
    # "npm install",
    # "docker compose up -d",
]

# [down] uses a list of shell commands run by `enx down`.
[down]
steps = [
    # "docker compose down",
]

# [start] uses a list of shell commands run by `enx start`.
[start]
commands = [
    # "npm run dev",
]

# [tasks] must use nested tables per task.
# valid:
#   [tasks.test]
#   command = "npm test"
#   description = "Run tests"
#
# invalid (will fail to parse):
#   [tasks]
#   test = "npm test"
#
# If the task name contains special characters (like ':'), quote the table key:
# [tasks."db:migrate"]
# command = "npx prisma migrate dev"
[tasks.test]
command = "echo \"define your test command\""
description = "Run test suite"

# [open] maps target name -> URL or shell command for `enx open <target>`.
[open]
# repo = "https://github.com/myorg/my-repo"
# code = "code ."
# docs = "https://docs.example.com"
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
        // Skip stale entries whose directories no longer exist on disk —
        // is_same_file returns an IO error when either path is missing.
        if !path.exists() {
            continue;
        }
        if is_same_file(project_path.as_path(), path)? {
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
