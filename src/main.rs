use std::path::PathBuf;

use clap::{Parser, Subcommand};
use env_logger::{Builder, Env};
use log::{info, warn};

mod backup;
mod utils;

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = "Backs up a Vaultwarden instance with optional selective backups"
)]
#[command(
    help_template = "{name}\n{author-with-newline}{about-section}\n{usage-heading} {usage}\n\n{all-args} {tab}"
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
    fn run(&self, args: Option<PathBuf>) {
        match self {
            Commands::Backup => {
                Builder::from_env(Env::default().default_filter_or("info"))
                    .format_timestamp_secs()
                    .init();

                info!("Application starting up...");

                let config = utils::read_config(args).unwrap();

                backup::controller::perform_backups(&config).unwrap();

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
            }
            Commands::GenerateConfig => {
                Builder::from_env(Env::default().default_filter_or("info"))
                    .format_timestamp_secs()
                    .init();

                info!("Application starting up...");

                let config = utils::read_config(args);

                config.unwrap();
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    cli.command.run(cli.config);
}
