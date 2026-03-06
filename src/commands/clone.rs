//! The `enx clone` command — clone a Git repository and initialize it as a project.
use std::path::PathBuf;
use std::process::Command;

use super::init;
use crate::config;
use crate::config::global;
use crate::output;
use crate::util;
use console::style;

/// Extract the repository name from a Git URL.
///
/// Handles HTTPS URLs (`https://github.com/user/repo.git`), SSH URLs
/// (`git@github.com:user/repo.git`), and bare names (`repo`). The trailing
/// `.git` suffix is stripped if present, and any trailing slashes are ignored.
fn repo_name_from_url(url: &str) -> String {
    url.trim_end_matches('/')
        .rsplit(['/', ':'])
        .next()
        .unwrap_or(url)
        .trim_end_matches(".git")
        .to_string()
}

pub fn run(repo: &str, path: Option<PathBuf>) -> anyhow::Result<()> {
    // Determine where the clone will land so we can pass it to `init::run`.
    let clone_path = if let Some(p) = &path {
        p.clone()
    } else {
        let global_config = global::GlobalConfig::load_from_file(&config::global_config_path()?)?;

        let base = match global_config.projects_dir() {
            Some(dir) => util::expand_tilde(&dir),
            None => std::env::current_dir()?,
        };

        base.join(repo_name_from_url(repo))
    };

    // Ensure the parent directory exists so git can create the clone target.
    if let Some(parent) = clone_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let sp = output::spinner(&format!("Cloning {} ...", style(repo).color256(116)));

    let result = Command::new("git")
        .args(["clone", repo])
        .arg(&clone_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .output();

    match result {
        Ok(out) if out.status.success() => {
            sp.finish_and_clear();
            output::step_ok("Repository cloned");
        }
        _ => {
            sp.finish_and_clear();
            output::step_fail("Clone failed");
            anyhow::bail!(
                "failed to clone repository. Make sure git is installed and you are authorized."
            );
        }
    }

    init::run(Some(clone_path))?;

    output::success("All done!");

    Ok(())
}
