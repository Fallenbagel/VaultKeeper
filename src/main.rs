use std::{env, path::PathBuf};

use clap::{Parser, Subcommand};

use sysops::scheduler::ActivationTime;
use tracing::{info, instrument, warn};

mod backup;
mod diagnostics;
mod sysops;
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
    /// Systemd timer to run backup
    #[cfg(target_os = "linux")]
    Install {
        /// Install systemd service and timer files for scheduled backups
        #[arg(short, long, value_name = "HH:MM or daily")]
        schedule: ActivationTime,

        /// generate systemd service and timer files in current directory
        #[arg(short, long)]
        manual: Option<bool>,

        /// Install binary to user's PATH
        #[arg(short, long)]
        binary: Option<bool>,
    },
}

impl Commands {
    #[instrument(skip(self, args))]
    fn run(&self, args: &Option<PathBuf>) -> Result<(), color_eyre::Report> {
        match self {
            Commands::Backup => {
                info!("Application starting up...");

                let (config, _config_path) = utils::read_config(&args)?;

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
                info!("Generating Config...");

                let _config = utils::read_config(&args)?;

                Ok(())
            }
            #[cfg(target_os = "linux")]
            Commands::Install {
                schedule,
                manual,
                binary,
            } => {
                if binary.unwrap_or(false) {
                    info!("Installing binary...");
                    sysops::installer::install_binary()?;
                }

                info!("Setting up systemd service and timer files...");

                let (_config, mut config_path) = utils::read_config(&args)?;

                if !manual.unwrap_or(false) {
                    config_path = PathBuf::from("/etc/vaultkeeper/config.json");
                }

                sysops::scheduler::create_systemd_service(&manual, &config_path)?;
                sysops::scheduler::create_systemd_timer(schedule, manual)?;

                Ok(())
            }
        }
    }
}

fn main() -> Result<(), color_eyre::Report> {
    diagnostics::setup()?;
    let cli = Cli::parse();
    cli.command.run(&cli.config)?;
    Ok(())
}
