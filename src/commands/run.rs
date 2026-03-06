//! The `enx run` command — execute a named task.

use std::collections::HashMap;
use std::path::Path;

use colored::Colorize;

use crate::config;
use crate::config::project::TaskConfig;
use crate::output;

pub fn run(task_name: Option<&str>, args: Vec<String>) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    // If no task name provided, list all available tasks
    if let None = task_name {
        return list_tasks(&current_dir);
    }

    let task_name = task_name.unwrap();
    let (task, env_vars) = resolve_task(task_name, &current_dir)?;

    output::info(&format!("Running task '{} {:?}'", task.command, args));

    let full_command = if args.is_empty() {
        task.command.clone()
    } else {
        format!("{} {}", task.command, args.join(" "))
    };

    execute_command(&full_command, &current_dir, &env_vars)
}

fn resolve_task(task_name: &str, project_dir: &Path) -> anyhow::Result<(TaskConfig, HashMap<String, String>)> {
    if let Ok(project_config) = config::project::ProjectConfig::load_from_file(project_dir) {
        let env_vars = collect_env_vars(&project_config);

        if let Some(task) = project_config
            .tasks
            .as_ref()
            .and_then(|tasks| tasks.get(task_name))
            .cloned()
        {
            return Ok((task, env_vars))
        }
    }

    let global_path = config::global_config_path()?;
    let global_config = config::global::GlobalConfig::load_from_file(&global_path)?;
    
    if let Some(task) = global_config
        .tasks
        .as_ref()
        .and_then(|tasks| tasks.get(task_name))
        .cloned()
    {
        return Ok((task, HashMap::new())) // Global tasks don't support envs for now.
    }

    anyhow::bail!{
        "Task '{}' not found in the current project or in global config. Are you sure you're in the right project?",
        task_name
    };
}

pub fn collect_env_vars(config: &config::project::ProjectConfig) -> HashMap<String, String> {
    let mut vars = HashMap::new();

    if let Some(env_config) = &config.env {
        if let Some(environments) = &env_config.environments {
            for (key, value) in environments {
                vars.insert(key.clone(), value.clone());
            }
        }
    }

    vars.insert("ENX_PROJECT".to_string(), config.project.name.clone());

    vars
}

pub fn execute_command(command: &str, working_dir: &Path, env_vars: &HashMap<String, String>) -> anyhow::Result<()> {
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let status = std::process::Command::new(shell)
        .arg(flag)
        .arg(command)
        .current_dir(working_dir)
        .envs(env_vars)
        .status()?;

    if !status.success() {
        let code = status.code().unwrap_or(-1);
        anyhow::bail!("Command '{}' exited with code {}", command, code);
    };

    Ok(())
}

fn list_tasks(project_dir: &Path) -> anyhow::Result<()> {
    let mut has_tasks = false;

    // Load and display project tasks
    if let Ok(project_config) = config::project::ProjectConfig::load_from_file(project_dir) {
        if let Some(tasks) = &project_config.tasks {
            if !tasks.is_empty() {
                has_tasks = true;
                output::header("Project Tasks:");
                for (name, task) in tasks {
                    print_task_entry(name, task);
                }
                println!();
            }
        }
    }

    // Load and display global tasks
    if let Ok(global_path) = config::global_config_path() {
        if let Ok(global_config) = config::global::GlobalConfig::load_from_file(&global_path) {
            if let Some(tasks) = &global_config.tasks {
                if !tasks.is_empty() {
                    has_tasks = true;
                    output::header("Global Tasks:");
                    for (name, task) in tasks {
                        print_task_entry(name, task);
                    }
                    println!();
                }
            }
        }
    }

    if !has_tasks {
        output::warning("No tasks found. Define tasks in your project's enx.toml or global config.");
    }

    Ok(())
}

fn print_task_entry(name: &str, task: &TaskConfig) {
    let description = task
        .description
        .as_ref()
        .map(|d| d.as_str())
        .unwrap_or("(no description)");
    output::detail(&format!("{}: {} | runs: {}", name.bold(), description, task.command));
}
