use std::path::PathBuf;

use dialoguer::Input;

use crate::commands::shell_init;
use crate::config;
use crate::output;

pub fn run() -> anyhow::Result<()> {
    let global_path = config::global_config_path()?;
    let mut global_config = config::global::GlobalConfig::load_from_file(&global_path)?;

    if global_config.is_setup_complete() {
        output::warning(
            "Setup has already been completed. Note that proceeding will overwrite your previous settings.",
        );
    }

    output::info("Welcome to the enx setup! Let's get you configured.");

    // Ask for projects directory
    let default_dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("enx-projects")
        .to_str()
        .unwrap()
        .to_string();

    let projects_dir: String = Input::new()
        .with_prompt("Where would you like to store your projects? (You can change this later in the config file if needed).")
        .default(default_dir.clone())
        .interact_text()?;

    let final_projects_dir = if projects_dir.is_empty() {
        default_dir
    } else {
        projects_dir
    };

    initialize_shell()?;

    // Save the configuration
    global_config.defaults = Some(config::global::DefaultsConfig {
        projects_dir: Some(final_projects_dir),
    });
    global_config.is_configured = true;

    let toml_string = toml::to_string_pretty(&global_config)?;
    std::fs::write(global_path, toml_string)?;

    output::success("Setup complete! You can now start using enx to manage your projects.");

    Ok(())
}

fn initialize_shell() -> anyhow::Result<()> {
    let shell = detect_shell()?;
    let script = shell_init::generate_script(&shell)?;

    let shell_dir = config::config_dir()?.join("shell");
    std::fs::create_dir_all(&shell_dir)?;

    let script_path = shell_dir.join(script_file_name(&shell));
    std::fs::write(&script_path, script)?;

    let rc_path = shell_rc_path(&shell)?;
    let source_line = source_line(&shell, &script_path);
    ensure_source_line(&rc_path, &source_line)?;

    output::info(&format!(
        "Shell integration installed for {shell} ({})",
        rc_path.display()
    ));

    Ok(())
}

fn detect_shell() -> anyhow::Result<String> {
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
        "zsh" => home.join(".zshrc"),
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
