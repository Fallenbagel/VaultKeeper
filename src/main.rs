use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::{info, instrument, warn};

mod backup;
mod diagnostics;
mod utils;

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = "Backs up a Vaultwarden instance with optional selective backups"
)]

struct Cli {
    /// Run backup
    #[command(subcommand)]
    command: Commands,

    /// Specify a custom path to generate/read the config file
    #[arg(short, long, value_name = "PATH", global = true)]
    config: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run backup
    Backup,
    /// Generate config file with default values.
    GenerateConfig,
}

impl Commands {
    #[instrument(skip(self, args))]
    fn run(&self, args: Option<PathBuf>) -> Result<(), color_eyre::Report> {
        match self {
            Commands::Backup => {
                info!("Application starting up...");

                let config = utils::read_config(args)?;

                backup::controller::perform_backups(&config)?;

                if config.backup_db
                    || config.backup_attachments
                    || config.backup_config
                    || config.backup_rsa_keys
                    || config.backup_icon_cache
                    || config.backup_sends
                {
                    backup::utils::manage_backups(&config);
                } else {
                    warn!("No backups to perform/manage");
                }

                info!("Application finished");
                Ok(())
            }
            Commands::GenerateConfig => {
                info!("Application starting up...");

                let _config = utils::read_config(args)?;

                Ok(())
            }
        }
    }
}

fn main() -> Result<(), color_eyre::Report> {
    diagnostics::setup()?;
    let cli = Cli::parse();
    cli.command.run(cli.config)?;
    Ok(())
}
