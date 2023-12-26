use std::{
    path::{Path, PathBuf},
    process::Command,
};

use log::{error, info};

use crate::utils::Config;

pub fn backup_sends(config: &Config, backup_dir: &Path) -> Result<(), String> {
    info!("Backing up sends folder");

    let source = PathBuf::from(&config.source_dir).join("sends");

    let output = Command::new("rsync")
        .arg("-a")
        .arg(source)
        .arg(backup_dir)
        .output()
        .map_err(|e| format!("Failed to execute rsync: {}", e))?;

    if output.status.success() {
        info!("Sends folder backed up");
        Ok(())
    } else {
        error!("Failed to backup sends folder. rsync output: {:?}", output);
        Err(format!(
            "Failed to backup sends folder. rsync output: {:?}",
            output
        ))
    }
}
