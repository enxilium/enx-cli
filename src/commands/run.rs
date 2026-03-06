//! The `enx run` command — execute a named task.

use std::collections::HashMap;
use std::path::Path;

use console::style;

use crate::config;
use crate::config::project::TaskConfig;
use crate::output;

pub fn run(task_name: Option<&str>, args: Vec<String>) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    // If no task name provided, list all available tasks
    if task_name.is_none() {
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

fn resolve_task(
    task_name: &str,
    project_dir: &Path,
) -> anyhow::Result<(TaskConfig, HashMap<String, String>)> {
    match config::project::ProjectConfig::load_from_file(project_dir) {
        Ok(project_config) => {
            let env_vars = collect_env_vars(&project_config);

            if let Some(task) = project_config
                .tasks
                .as_ref()
                .and_then(|tasks| tasks.get(task_name))
                .cloned()
            {
                return Ok((task, env_vars));
            }
        }
        Err(e) => {
            // If the file exists but is malformed, warn the user so they know
            // their config has problems — then fall through to global tasks.
            if project_dir.join("enx.toml").exists() {
                output::warning(&format!("failed to parse enx.toml: {e}"));
            }
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
        return Ok((task, HashMap::new())); // Global tasks don't support envs for now.
    }

    anyhow::bail! {
        "Task '{}' not found in the current project or in global config. Are you sure you're in the right project?",
        task_name
    };
}

pub fn collect_env_vars(config: &config::project::ProjectConfig) -> HashMap<String, String> {
    let mut vars = HashMap::new();

    if let Some(env_config) = &config.env
        && let Some(environments) = &env_config.environments
    {
        for (key, value) in environments {
            vars.insert(key.clone(), value.clone());
        }
    }

    vars.insert("ENX_PROJECT".to_string(), config.project.name.clone());

    vars
}

pub fn execute_command(
    command: &str,
    working_dir: &Path,
    env_vars: &HashMap<String, String>,
) -> anyhow::Result<()> {
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

/// Run a command with a spinner that resolves to ✓ or ✗.
///
/// Unlike [`execute_command`], stdout and stderr are captured (not
/// inherited) so the spinner animation isn't disrupted by subprocess
/// output. On failure the captured stderr is printed below the ✗ line.
pub fn execute_command_with_spinner(
    command: &str,
    working_dir: &Path,
    env_vars: &HashMap<String, String>,
) -> anyhow::Result<()> {
    let sp = crate::output::spinner(command);

    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let output = std::process::Command::new(shell)
        .arg(flag)
        .arg(command)
        .current_dir(working_dir)
        .envs(env_vars)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()?;

    sp.finish_and_clear();

    if output.status.success() {
        crate::output::step_ok(command);
    } else {
        crate::output::step_fail(command);
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.trim().is_empty() {
            eprintln!("{}", stderr.trim());
        }
        let code = output.status.code().unwrap_or(-1);
        anyhow::bail!("command '{}' exited with code {}", command, code);
    }

    Ok(())
}

fn list_tasks(project_dir: &Path) -> anyhow::Result<()> {
    let mut has_tasks = false;

    // Load and display project tasks
    match config::project::ProjectConfig::load_from_file(project_dir) {
        Ok(project_config) if project_config.tasks.as_ref().is_some_and(|t| !t.is_empty()) => {
            let tasks = project_config.tasks.as_ref().unwrap();
            has_tasks = true;
            output::header("Project Tasks");
            let items: Vec<_> = tasks.iter().collect();
            let last = items.len().saturating_sub(1);
            for (i, (name, task)) in items.iter().enumerate() {
                print_task_entry(name, task, i == last);
            }
            output::newline();
        }
        Err(e) if project_dir.join("enx.toml").exists() => {
            output::warning(&format!("failed to parse enx.toml: {e}"));
        }
        _ => {}
    }

    // Load and display global tasks
    if let Ok(global_path) = config::global_config_path()
        && let Ok(global_config) = config::global::GlobalConfig::load_from_file(&global_path)
        && let Some(tasks) = &global_config.tasks
        && !tasks.is_empty()
    {
        has_tasks = true;
        output::header("Global Tasks");
        let items: Vec<_> = tasks.iter().collect();
        let last = items.len().saturating_sub(1);
        for (i, (name, task)) in items.iter().enumerate() {
            print_task_entry(name, task, i == last);
        }
        output::newline();
    }

    if !has_tasks {
        output::warning(
            "No tasks found. Define tasks in your project's enx.toml or global config.",
        );
    }

    Ok(())
}

fn print_task_entry(name: &str, task: &TaskConfig, is_last: bool) {
    let description = task.description.as_deref().unwrap_or("(no description)");
    let connector = if is_last { "└─" } else { "├─" };
    println!(
        "  {} {} {} {}",
        style(connector).color256(102),
        style(name).color256(116).bold(),
        style("─").color256(102),
        style(description).color256(102),
    );
}
