use std::{
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::eyre::eyre;
use tracing::{error, info};

use crate::utils::Config;

pub fn backup_sends(config: &Config, backup_dir: &Path) -> Result<(), color_eyre::Report> {
    info!("Backing up sends folder");

    let source = PathBuf::from(&config.source_dir).join("sends");

    let output = Command::new("rsync")
        .arg("-a")
        .arg(source)
        .arg(backup_dir)
        .output()
        .map_err(|e| eyre!("Failed to execute rsync: {}", e))?;

    if output.status.success() {
        info!("Sends folder backed up");
        Ok(())
    } else {
        error!("Failed to backup sends folder. rsync output: {:?}", output);
        Err(eyre!(
            "Failed to backup sends folder. rsync output: {:?}",
            output
        ))
    }
}
