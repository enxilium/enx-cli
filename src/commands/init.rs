//! The `enx init` command. Used to initialize a new project and add it to the registry.

use crate::config;
use crate::output;
use same_file::is_same_file;
use std::path::PathBuf;

/// Generate a default `enx.toml` with the project name filled in and all
/// sections present but empty.
fn default_enx_toml(project_name: &str) -> String {
    format!(
        r#"# enx project configuration
# syntax help: this file is TOML. strings are quoted, arrays use [..], and tables use [table].

[project]
# display name shown in `enx projects`
name = "{project_name}"

# Environment file mappings used by `enx env <name>`.
# Each key is an environment name, each value is a dotenv file path.
# Example:
# [env]
# dev = ".env"
# staging = ".env.staging"
[env]
# dev = ".env"

# Commands run by `enx up`.
# Use a TOML array of shell command strings.
[up]
steps = [
    # "npm install",
    # "docker compose up -d",
]

# Commands run by `enx down`.
[down]
steps = [
    # "docker compose down",
]

# Commands run by `enx start`.
# Multiple commands run in order.
[start]
commands = [
    # "npm run dev",
]

# Task definitions for `enx run <task>` or shorthand `enx <task>`.
# Syntax:
# [tasks.<name>]
# command = "..."
# description = "..."
#
# If the task name contains special characters (like ':'), quote it:
# [tasks."db:migrate"]
# command = "npx prisma migrate dev"
[tasks]

[tasks.test]
command = "echo \"define your test command\""
description = "Run test suite"

# Named targets for `enx open <target>`.
# Values can be URLs or shell commands.
# Example targets: repo, docs, code, ci
[open]
# repo = "https://github.com/myorg/my-repo"
# code = "code ."
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
