use std::path::PathBuf;

/// Expand a leading `~` or `~/` in a path string to the user's home directory.
///
/// Shells normally handle tilde expansion, but when reading raw user input
/// (e.g. from `dialoguer`) or from config files, `~` is just a literal
/// character. This function resolves it so file operations work as the
/// user expects.
///
/// If there is no leading `~`, the string is returned unchanged as a `PathBuf`.
pub fn expand_tilde(path: &str) -> PathBuf {
    if path == "~" {
        return dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    }

    if let Some(rest) = path.strip_prefix("~/")
        && let Some(home) = dirs::home_dir()
    {
        return home.join(rest);
    }

    PathBuf::from(path)
}
