mod cli;
mod commands;
mod config;
mod output;
mod shell;

use clap::Parser;

fn main() {
    let result = cli::Cli::try_parse();

    let cli = match result {
        Ok(cli) => cli,
        Err(e) => {
            if matches!(
                e.kind(),
                clap::error::ErrorKind::InvalidSubcommand | clap::error::ErrorKind::UnknownArgument
            ) {
                let raw_args: Vec<String> = std::env::args().skip(1).collect();

                if let Some(task_name) = raw_args.first() {
                    let task_args = &raw_args[1..];

                    if let Err(e) = commands::run::run(Some(task_name.as_str()), task_args.to_vec())
                    {
                        output::error(&format!("{:?}", e));
                        std::process::exit(1);
                    }
                    return;
                }
            }

            e.exit();
        }
    };

    if let Err(e) = dispatch(cli) {
        output::error(&format!("{:?}", e));
        std::process::exit(1);
    }
}

fn dispatch(cli: cli::Cli) -> anyhow::Result<()> {
    match cli.command {
        cli::Commands::Projects => commands::projects::run()?,
        cli::Commands::Init { path } => commands::init::run(path)?,
        cli::Commands::Remove { name } => commands::remove::run(&name)?,
        cli::Commands::Clone { repo, path } => commands::clone::run(&repo, path)?,
        cli::Commands::Cd { name } => commands::cd::run(&name)?,
        cli::Commands::Run { task, args } => commands::run::run(task.as_deref(), args)?,
        cli::Commands::Start => commands::start::run()?,
        cli::Commands::Up => commands::up::run()?,
        cli::Commands::Down => commands::down::run()?,
        cli::Commands::Open { target } => commands::open::run(&target)?,
        cli::Commands::Env { query } => commands::env::run(query.as_deref())?,
        cli::Commands::Setup => commands::setup::run()?,

        _ => {
            output::warning("This command has not been implemented yet.");
        }
    }

    Ok(())
}
