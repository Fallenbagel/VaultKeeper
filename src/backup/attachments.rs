use std::{
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::eyre::eyre;
use tracing::{error, info};

use crate::utils::Config;

pub fn backup_attachments(config: &Config, backup_dir: &Path) -> Result<(), color_eyre::Report> {
    info!("Backing up attachments");

    let source = PathBuf::from(&config.source_dir).join("attachments");

    let output = Command::new("rsync")
        .arg("-a")
        .arg(source)
        .arg(backup_dir)
        .output()
        .map_err(|e| eyre!("Failed to execute rsync: {}", e))?;

    if output.status.success() {
        info!("Attachments backed up");
        Ok(())
    } else {
        error!("Failed to backup attachments. rsync output: {:?}", output);
        Err(eyre!(
            "Failed to backup attachments. rsync output: {:?}",
            output
        ))
    }
}
