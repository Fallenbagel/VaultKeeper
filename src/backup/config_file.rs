use std::{
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::eyre::eyre;
use tracing::{error, info};

use crate::utils::Config;

pub fn backup_config_file(config: &Config, backup_dir: &Path) -> Result<(), color_eyre::Report> {
    info!("Backing up config file");

    let source = PathBuf::from(&config.source_dir).join("config.json");
    let backup_file = backup_dir.join("config.json");

    let output = Command::new("rsync")
        .arg("-a")
        .arg(source)
        .arg(backup_file)
        .output()
        .map_err(|e| eyre!("Failed to execute rsync: {}", e))?;

    if output.status.success() {
        info!("Config file backed up");
        Ok(())
    } else {
        error!("Failed to backup config file. rsync output: {:?}", output);
        Err(eyre!(
            "Failed to backup config file. rsync output: {:?}",
            output
        ))
    }
}
