//! The finalizer system - shell integration.

use std::fmt;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Finalizer {
    Cd(PathBuf),
    SetEnv { key: String, value: String },
}

impl fmt::Display for Finalizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Finalizer::Cd(path) => write!(f, "cd:{}", path.display()),
            Finalizer::SetEnv { key, value } => write!(f, "setenv:{}={}", key, value),
        }
    }
}

pub fn write_finalizers(finalizer: &[Finalizer]) -> anyhow::Result<()> {
    // &[x] is a borrowed slice, a borrowed view into a continguous sequence of x (e.g. Vector, array, etc.)
    // Idiomatic to String vs. &str or PathBuf vs. &Path

    let path = match std::env::var("ENX_FINALIZER_FILE") {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            return Ok(());
        }
    };

    let mut file = std::fs::File::create(&path)?;

    for f in finalizer {
        writeln!(file, "{}", f)?;
    }

    Ok(())
}
