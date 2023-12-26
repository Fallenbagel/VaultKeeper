use std::{
    path::{Path, PathBuf},
    process::Command,
};

use log::{error, info};

use crate::utils::Config;

pub fn backup_attachments(config: &Config, backup_dir: &Path) -> Result<(), String> {
    info!("Backing up attachments");

    let source = PathBuf::from(&config.source_dir).join("attachments");

    let output = Command::new("rsync")
        .arg("-a")
        .arg(source)
        .arg(backup_dir)
        .output()
        .map_err(|e| format!("Failed to execute rsync: {}", e))?;

    if output.status.success() {
        info!("Attachments backed up");
        Ok(())
    } else {
        error!("Failed to backup attachments. rsync output: {:?}", output);
        Err(format!(
            "Failed to backup attachments. rsync output: {:?}",
            output
        ))
    }
}
