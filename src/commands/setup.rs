use std::path::PathBuf;

use dialoguer::{Confirm, Input};

use crate::commands::init;
use crate::commands::shell_init;
use crate::config;
use crate::output;
use crate::util;

pub fn run() -> anyhow::Result<()> {
    let global_path = config::global_config_path()?;
    let mut global_config = config::global::GlobalConfig::load_from_file(&global_path)?;

    // ── Banner ───────────────────────────────────────────────────────
    output::banner();

    if global_config.is_setup_complete() {
        output::warning(
            "Setup has already been completed. Note that proceeding will overwrite your previous settings.",
        );
        output::newline();
    }

    // ── Projects directory ───────────────────────────────────────────
    output::header("Project Directory");
    let default_dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("enx-projects")
        .to_str()
        .unwrap()
        .to_string();

    let projects_dir: String = Input::with_theme(&output::theme())
        .with_prompt("Where would you like to store your projects?")
        .default(default_dir.clone())
        .interact_text()?;

    let final_projects_dir = if projects_dir.is_empty() {
        default_dir
    } else {
        projects_dir
    };

    // Expand ~ to the real home path so file operations work correctly.
    let expanded_dir = util::expand_tilde(&final_projects_dir);
    let final_projects_dir = expanded_dir
        .to_str()
        .unwrap_or(&final_projects_dir)
        .to_string();

    output::newline();

    // ── Auto-index ──────────────────────────────────────────────────
    auto_index_projects(&final_projects_dir)?;

    // ── Shell integration ───────────────────────────────────────────
    output::header("Shell Integration");
    initialize_shell()?;

    output::newline();

    // ── Save & finish ───────────────────────────────────────────────
    global_config.defaults = Some(config::global::DefaultsConfig {
        projects_dir: Some(final_projects_dir),
    });
    global_config.is_configured = true;

    let toml_string = toml::to_string_pretty(&global_config)?;
    std::fs::write(global_path, toml_string)?;

    output::success("Setup complete! You're all set.");

    Ok(())
}

/// Scan the projects directory for existing subdirectories and offer to
/// register each one via `enx init`. Only immediate children that are
/// directories are considered.
fn auto_index_projects(projects_dir: &str) -> anyhow::Result<()> {
    let path = PathBuf::from(projects_dir);

    if !path.is_dir() {
        return Ok(());
    }

    let subdirs: Vec<_> = std::fs::read_dir(&path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .collect();

    if subdirs.is_empty() {
        return Ok(());
    }

    let names: Vec<_> = subdirs
        .iter()
        .filter_map(|e| e.file_name().to_str().map(String::from))
        .collect();

    output::header("Auto-Index Projects");
    output::info(&format!(
        "Found {} subdirector{} in '{}':",
        names.len(),
        if names.len() == 1 { "y" } else { "ies" },
        projects_dir
    ));

    let last = names.len().saturating_sub(1);
    for (i, name) in names.iter().enumerate() {
        if i == last {
            output::detail_last(name);
        } else {
            output::detail(name);
        }
    }

    let should_index = Confirm::with_theme(&output::theme())
        .with_prompt("Register all of them as enx projects?")
        .default(true)
        .interact()?;

    if !should_index {
        output::newline();
        return Ok(());
    }

    for entry in &subdirs {
        let dir = entry.path();
        let name = entry.file_name().to_str().unwrap_or("unknown").to_string();

        match init::run(Some(dir.clone())) {
            Ok(()) => output::step_ok(&name),
            Err(e) => {
                output::step_fail(&name);
                output::warning(&format!("skipping '{}': {}", name, e));
            }
        }
    }

    output::newline();

    Ok(())
}

fn initialize_shell() -> anyhow::Result<()> {
    let shell = detect_shell()?;

    let sp = output::spinner(&format!("Installing shell integration for {shell}..."));

    let script = shell_init::generate_script(&shell)?;

    let shell_dir = config::config_dir()?.join("shell");
    std::fs::create_dir_all(&shell_dir)?;

    let script_path = shell_dir.join(script_file_name(&shell));
    std::fs::write(&script_path, script)?;

    let rc_path = shell_rc_path(&shell)?;
    let source_line = source_line(&shell, &script_path);
    ensure_source_line(&rc_path, &source_line)?;

    sp.finish_and_clear();

    output::step_ok(&format!(
        "Shell integration installed for {shell} ({})",
        rc_path.display()
    ));

    Ok(())
}

fn detect_shell() -> anyhow::Result<String> {
    // Check shell-specific environment variables first (detects the current running shell)
    if std::env::var("FISH_VERSION").is_ok() {
        return Ok("fish".to_string());
    }
    if std::env::var("ZSH_VERSION").is_ok() {
        return Ok("zsh".to_string());
    }
    if std::env::var("BASH_VERSION").is_ok() {
        return Ok("bash".to_string());
    }
    if std::env::var("POWERSHELL_DISTRIBUTION_CHANNEL").is_ok()
        || std::env::var("PSModulePath").is_ok()
    {
        return Ok("pwsh".to_string());
    }

    // Fallback to $SHELL (login shell default)
    let shell_path = std::env::var("SHELL").unwrap_or_default();
    let shell_name = std::path::Path::new(&shell_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let detected = match shell_name.as_str() {
        "bash" => "bash",
        "zsh" => "zsh",
        "fish" => "fish",
        "pwsh" | "powershell" => "pwsh",
        _ => {
            // On Windows, SHELL is usually unset. Prefer pwsh as the default.
            if cfg!(target_os = "windows") {
                "pwsh"
            } else {
                "bash"
            }
        }
    };

    Ok(detected.to_string())
}

fn shell_rc_path(shell: &str) -> anyhow::Result<PathBuf> {
    let home =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("could not determine home directory"))?;

    let path = match shell {
        "bash" => home.join(".bashrc"),
        "zsh" => {
            // Respect ZDOTDIR if set — zsh reads .zshrc from there instead of $HOME.
            match std::env::var("ZDOTDIR") {
                Ok(zdotdir) if !zdotdir.is_empty() => PathBuf::from(zdotdir).join(".zshrc"),
                _ => home.join(".zshrc"),
            }
        }
        "fish" => home.join(".config/fish/config.fish"),
        "pwsh" => {
            if cfg!(target_os = "windows") {
                home.join("Documents/PowerShell/Microsoft.PowerShell_profile.ps1")
            } else {
                home.join(".config/powershell/Microsoft.PowerShell_profile.ps1")
            }
        }
        _ => anyhow::bail!("unsupported shell: {shell}"),
    };

    Ok(path)
}

fn script_file_name(shell: &str) -> &'static str {
    match shell {
        "fish" => "init.fish",
        "pwsh" => "init.ps1",
        _ => "init.sh",
    }
}

fn source_line(shell: &str, script_path: &std::path::Path) -> String {
    let rendered = script_path.display();

    match shell {
        "fish" => format!("source {rendered}"),
        "pwsh" => format!(". \"{rendered}\""),
        _ => format!("[ -f \"{rendered}\" ] && source \"{rendered}\""),
    }
}

fn ensure_source_line(rc_path: &std::path::Path, source_line: &str) -> anyhow::Result<()> {
    if let Some(parent) = rc_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let existing = if rc_path.exists() {
        std::fs::read_to_string(rc_path)?
    } else {
        String::new()
    };

    if existing.contains(source_line) {
        return Ok(());
    }

    let mut updated = existing;
    if !updated.is_empty() && !updated.ends_with('\n') {
        updated.push('\n');
    }
    updated.push_str(source_line);
    updated.push('\n');

    std::fs::write(rc_path, updated)?;
    Ok(())
}
