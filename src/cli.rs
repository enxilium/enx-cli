use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// enx - A cross-platform developer workflow CLI.
#[derive(Debug, Parser)]
#[command(name = "enx")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Show all available enx commands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Navigate to a project directory by name.
    Cd {
        /// Name of the project to navigate to.
        name: String,
    },

    /// Initializes and sets up the project environment. Must be inside a registered project directory.
    Up,

    /// Tears down the project environment. Must be inside a registered project directory.
    Down,

    /// Starts the current project as defined per-project. Must be inside a registered project directory.
    Start,

    /// Stops the current project as defined per-project. Must be inside a registered project directory.
    Stop,

    /// Displays active diagnostic information about the project.
    Doctor,

    /// Lists all registered projects.
    Projects,

    /// Initializes a new project, adding it to the registry and creating an enx.toml file with default settings. Can be run anywhere.
    Init {
        /// Optional path parameter. Defaults to current directory.
        path: Option<PathBuf>,
    },

    /// Removes a project from the registry. Can choose whether to also delete the directory from filesystem.
    Remove {
        /// Name of the project to remove from the registry.
        name: String,
    },

    /// Clones a remote repository and adds it to the registry.
    Clone {
        repo: String,
        /// Optional path parameter to clone to. Defaults to project directory in global settings.
        path: Option<PathBuf>,
    },

    /// Switches to a different environment in the current project. Environments are defined in the project's enx.toml file and can represent different configurations, e.g. for development, staging, production, etc.
    /// If no environment name is provided, lists all available environments.
    Env {
        /// Name of the environment to switch to. Optional; if omitted, lists all environments.
        query: Option<String>,
    },

    /// Opens a URL in the default browser. Targets are defined in the project's enx.toml under [open] and must be URLs.
    Open { target: String },

    /// Shows current project's status.
    Status,

    /// Runs a custom task defined in the project's enx.toml file under the [tasks] section, or global tasks defined in enx config.
    /// If no task name is provided, displays all available tasks.
    Run {
        /// Name of the task to run. Optional; if omitted, lists all available tasks.
        task: Option<String>,

        /// Optional additional arguments to pass to the task command. Depends on task.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Performs initial setup for enx, including creating global configuration and setting up shell integrations. Should be run once after installing enx.
    Setup,
}
