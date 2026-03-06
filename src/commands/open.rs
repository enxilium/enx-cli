//! The `enx open` command — open a URL in the default browser.
//!
//! Every target under `[open]` in `enx.toml` must be a URL (starting with
//! `http://` or `https://`). The URL is opened in the OS default browser
//! via the `open` crate.

use crate::config;
use crate::output;

pub fn run(target: &str) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let project_config = config::project::ProjectConfig::load_from_file(&current_dir)?;

    let open_config = project_config
        .open
        .ok_or_else(|| anyhow::anyhow!("no [open] section found in enx.toml"))?;

    let url = open_config.get(target).ok_or_else(|| {
        anyhow::anyhow!(
            "no open target '{}' found in enx.toml. Available targets: {}",
            target,
            open_config.keys().cloned().collect::<Vec<_>>().join(", ")
        )
    })?;

    if !url.starts_with("http://") && !url.starts_with("https://") {
        anyhow::bail!(
            "open target '{}' must be a URL (starting with http:// or https://), got: {}",
            target,
            url
        );
    }

    output::info(&format!("Opening {} in browser...", url));
    open::that(url)?;

    Ok(())
}
