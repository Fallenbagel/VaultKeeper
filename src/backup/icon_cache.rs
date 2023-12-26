use std::{
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::eyre::eyre;
use tracing::{error, info};

use crate::utils::Config;

pub fn backup_icon_cache(config: &Config, backup_dir: &Path) -> Result<(), color_eyre::Report> {
    info!("Backing up icon cache folder");

    let source = PathBuf::from(&config.source_dir).join("icon_cache");

    let output = Command::new("rsync")
        .arg("-a")
        .arg(source)
        .arg(backup_dir)
        .output()
        .map_err(|e| eyre!("Failed to execute rsync: {}", e))?;

    if output.status.success() {
        info!("Icon cache folder backed up");
        Ok(())
    } else {
        error!(
            "Failed to backup icon cache folder. rsync output: {:?}",
            output
        );
        Err(eyre!(
            "Failed to backup icon cache folder. rsync output: {:?}",
            output
        ))
    }
}
