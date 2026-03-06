//! The `enx clone` command — clone a Git repository and initialize it as a project.
use std::path::PathBuf;
use std::process::Command;

use super::init;
use crate::config;
use crate::config::global;
use crate::output;

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
            Some(dir) => PathBuf::from(dir),
            None => std::env::current_dir()?,
        };

        base.join(repo_name_from_url(repo))
    };

    let mut cmd = Command::new("git");
    cmd.args(["clone", repo]);
    cmd.arg(&clone_path);

    let status = cmd.status()?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "Failed to clone repository. Please make sure git is installed properly and you are authorized."
        ));
    }

    output::success("Repository cloned successfully. Running init...");

    init::run(Some(clone_path))?;

    output::success("All done!");

    Ok(())
}
