use std::{
    path::{Path, PathBuf},
    process::Command,
};

use log::{error, info};

use crate::utils::Config;

pub fn backup_icon_cache(config: &Config, backup_dir: &Path) -> Result<(), String> {
    info!("Backing up icon cache folder");

    let source = PathBuf::from(&config.source_dir).join("icon_cache");

    let output = Command::new("rsync")
        .arg("-a")
        .arg(source)
        .arg(backup_dir)
        .output()
        .map_err(|e| format!("Failed to execute rsync: {}", e))?;

    if output.status.success() {
        info!("Icon cache folder backed up");
        Ok(())
    } else {
        error!(
            "Failed to backup icon cache folder. rsync output: {:?}",
            output
        );
        Err(format!(
            "Failed to backup icon cache folder. rsync output: {:?}",
            output
        ))
    }
}
