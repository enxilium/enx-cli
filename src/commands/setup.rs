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

    // ── Windows PATH setup ──────────────────────────────────────────
    #[cfg(target_os = "windows")]
    {
        output::header("Windows PATH Setup");
        ensure_binary_in_path()?;
        output::newline();
    }

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

fn has_non_empty_env(key: &str) -> bool {
    std::env::var(key)
        .ok()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn normalize_shell_name(shell: &str) -> Option<&'static str> {
    match shell.trim().to_ascii_lowercase().as_str() {
        "fish" => Some("fish"),
        "zsh" => Some("zsh"),
        "bash" => Some("bash"),
        _ => None,
    }
}

#[cfg(unix)]
fn detect_parent_shell() -> Option<String> {
    use std::process::Command;

    let mut pid = std::os::unix::process::parent_id();

    for _ in 0..8 {
        if pid == 0 {
            return None;
        }

        let output = Command::new("ps")
            .arg("-p")
            .arg(pid.to_string())
            .arg("-o")
            .arg("comm=")
            .arg("-o")
            .arg("ppid=")
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let line = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if line.is_empty() {
            return None;
        }

        let mut parts = line.split_whitespace();
        let comm = parts.next()?.to_ascii_lowercase();
        let next_pid = parts
            .next()
            .and_then(|p| p.parse::<u32>().ok())
            .unwrap_or(0);

        let base = std::path::Path::new(&comm)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&comm)
            .trim_start_matches('-');

        match base {
            "fish" | "zsh" | "bash" => return Some(base.to_string()),
            _ => {
                pid = next_pid;
            }
        }
    }

    None
}

#[cfg(not(unix))]
fn detect_parent_shell() -> Option<String> {
    None
}

/// Detect the current shell using shell-specific version variables.
///
/// Priority follows active-shell markers first (`ZSH_VERSION`,
/// `BASH_VERSION`, `FISH_VERSION`), then falls back to `$SHELL`.
fn detect_shell() -> anyhow::Result<String> {
    if let Ok(explicit_shell) = std::env::var("ENX_SETUP_SHELL")
        && let Some(shell) = normalize_shell_name(&explicit_shell)
    {
        return Ok(shell.to_string());
    }

    if has_non_empty_env("FISH_VERSION") {
        return Ok("fish".to_string());
    }
    if has_non_empty_env("ZSH_VERSION") {
        return Ok("zsh".to_string());
    }
    if has_non_empty_env("BASH_VERSION") {
        return Ok("bash".to_string());
    }

    if let Some(parent_shell) = detect_parent_shell() {
        return Ok(parent_shell);
    }

    // Fallback: check $SHELL
    let shell_path = std::env::var("SHELL").unwrap_or_default();
    let shell_name = std::path::Path::new(&shell_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match shell_name.as_str() {
        "fish" => Ok("fish".to_string()),
        "zsh" => Ok("zsh".to_string()),
        "bash" => Ok("bash".to_string()),
        _ => anyhow::bail!("unsupported shell for setup; run 'enx setup' from bash, zsh, or fish"),
    }
}

fn shell_rc_path(shell: &str) -> anyhow::Result<PathBuf> {
    let home =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("could not determine home directory"))?;

    let path = match shell {
        "fish" => {
            let config_home = std::env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home.join(".config"));
            config_home.join("fish").join("config.fish")
        }
        "zsh" => {
            // Respect ZDOTDIR if set — zsh reads .zshrc from there instead of $HOME.
            match std::env::var("ZDOTDIR") {
                Ok(zdotdir) if !zdotdir.is_empty() => PathBuf::from(zdotdir).join(".zshrc"),
                _ => home.join(".zshrc"),
            }
        }
        // bash (and Git Bash on Windows)
        _ => home.join(".bashrc"),
    };

    Ok(path)
}

fn script_file_name(shell: &str) -> &'static str {
    match shell {
        "fish" => "init.fish",
        _ => "init.sh",
    }
}

fn source_line(shell: &str, script_path: &std::path::Path) -> String {
    let rendered = script_path.display();
    match shell {
        "fish" => format!("test -f \"{rendered}\"; and source \"{rendered}\""),
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

    let mut found_current = false;
    let mut kept_lines: Vec<&str> = Vec::new();

    for line in existing.lines() {
        let trimmed = line.trim();
        if trimmed == source_line {
            found_current = true;
            kept_lines.push(line);
            continue;
        }

        if is_enx_shell_source_line(trimmed) {
            continue;
        }

        kept_lines.push(line);
    }

    let mut updated = kept_lines.join("\n");
    if !updated.is_empty() && !updated.ends_with('\n') {
        updated.push('\n');
    }
    if !found_current {
        updated.push_str(source_line);
        updated.push('\n');
    }

    std::fs::write(rc_path, updated)?;
    Ok(())
}

fn is_enx_shell_source_line(line: &str) -> bool {
    let has_source = line.contains("source ");
    let has_init = line.contains("/enx/shell/init.sh")
        || line.contains("/enx/shell/init.fish")
        || line.contains("\\enx\\shell\\init.sh")
        || line.contains("\\enx\\shell\\init.fish");

    has_source && has_init
}

/// On Windows, ensure the binary's directory is in the user PATH.
/// This allows `enx` to be invoked from any PowerShell window without full path.
#[cfg(target_os = "windows")]
fn ensure_binary_in_path() -> anyhow::Result<()> {
    use std::process::Command;

    // Get the directory containing the running binary
    let exe_path = std::env::current_exe()?;
    let bin_dir = exe_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("could not determine binary directory"))?;

    let bin_dir_str = bin_dir.to_string_lossy().to_string();

    // Retrieve current user PATH
    let output = Command::new("powershell.exe")
        .arg("-NoProfile")
        .arg("-Command")
        .arg("[Environment]::GetEnvironmentVariable('PATH', 'User')")
        .output()?;

    let current_path = String::from_utf8(output.stdout)?;
    let path_entries: Vec<&str> = current_path.trim().split(';').collect();

    // Check if bin_dir is already in PATH
    if path_entries.iter().any(|entry| {
        entry.to_lowercase() == bin_dir_str.to_lowercase()
            || entry.trim().to_lowercase() == bin_dir_str.to_lowercase()
    }) {
        output::step_ok("enx binary directory already in PATH");
        return Ok(());
    }

    // Add to PATH using PowerShell to set the environment variable persistently
    let new_path = format!("{};{}", current_path.trim(), bin_dir_str);
    let cmd = format!(
        r#"[Environment]::SetEnvironmentVariable('PATH', '{}', 'User')"#,
        new_path.replace('\\', "\\\\").replace('"', "\\\"")
    );

    let output = Command::new("powershell.exe")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(&cmd)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap_or_default();
        anyhow::bail!("failed to update PATH: {}", stderr);
    }

    output::step_ok(&format!("Added {} to user PATH", bin_dir.display()));
    output::info("Restart your terminal for the PATH change to take effect");

    Ok(())
}
