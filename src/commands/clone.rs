//! The `enx clone` command — clone a Git repository and initialize it as a project.
use std::path::PathBuf;
use std::process::Command;

use crate::config::global;
use crate::output;
use crate::config;
use super::init;

pub fn run(repo: &str, path: Option<PathBuf>) -> anyhow::Result<()> {
    let mut cmd = Command::new("git");

    cmd.args(["clone", repo]);

    if let Some(p) = &path {
        cmd.arg(p);
    } else {
        let global_config = global::GlobalConfig::load_from_file(&config::global_config_path()?)?;

        if let Some(dir) = global_config.projects_dir() {
            cmd.arg(&dir);
        }
    }
    let status = cmd.status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to clone repository. Please make sure git is installed properly and you are authorized."));
    }

    output::success("Repository cloned successfully. Running init...");

    init::run(path)?;

    output::success("All done!");

    Ok(())
}