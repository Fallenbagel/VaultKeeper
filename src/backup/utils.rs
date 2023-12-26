use std::{
    fs::{create_dir_all, remove_dir_all},
    path::{Path, PathBuf},
};

use color_eyre::eyre::Context;
use tracing::{info, warn};

use crate::utils::Config;

pub fn create_backup_dir(config: &Config) -> Result<PathBuf, color_eyre::Report> {
    info!("Creating backup directory");
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let backup_path = Path::new(&config.backup_dir).join(timestamp);
    if !backup_path.exists() && config.backup_db
        || config.backup_attachments
        || config.backup_config
        || config.backup_rsa_keys
        || config.backup_icon_cache
        || config.backup_sends
    {
        create_dir_all(&backup_path)
            .wrap_err_with(|| format!("Failed to create backup directory: {:?}", backup_path))?;
        info!("Backup directory created: {:?}", backup_path);
    } else {
        warn!("Backup directory not created");
    }

    Ok(backup_path)
}

pub fn manage_backups(config: &Config) {
    info!("Managing backups");

    let backup_path = Path::new(&config.backup_dir);

    let mut backups: Vec<_> = backup_path
        .read_dir()
        .expect("Could not read backup directory")
        .map(|entry| entry.unwrap().path())
        .collect();

    backups.sort();

    let backup_count = config.backup_count as usize;

    if backups.len() > backup_count {
        let to_delete = backups.len() - backup_count;
        warn!("Deleting {} backups", to_delete);
        for backup in backups.iter().take(to_delete) {
            warn!("Deleting {:?}", backup);
            remove_dir_all(backup).expect("Could not delete backup");
        }
    } else {
        info!("No backups to delete");
    }
}
