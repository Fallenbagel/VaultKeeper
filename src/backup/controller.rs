use log::{error, info};

use super::{
    attachments::backup_attachments, config_file::backup_config_file, database::backup_database,
    icon_cache::backup_icon_cache, rsa_keys::backup_rsa_keys, sends::backup_sends,
    utils::create_backup_dir,
};

use crate::utils::Config;

pub fn perform_backups(config: &Config) -> Result<(), String> {
    if check_dirs_exist(config).is_err() {
        return Err("Source and/or backup directories are empty".to_string());
    }

    let backup_dir = create_backup_dir(config);

    if config.backup_db {
        info!("Backing up database");
        backup_database(config, &backup_dir)?;
    }

    if config.backup_attachments {
        info!("Backing up attachments");
        backup_attachments(config, &backup_dir)?;
    }

    if config.backup_config {
        info!("Backing up config file");
        backup_config_file(config, &backup_dir)?;
    }

    if config.backup_rsa_keys {
        info!("Backing up RSA keys");
        backup_rsa_keys(config, &backup_dir)?;
    }

    if config.backup_icon_cache {
        info!("Backing up icon cache");
        backup_icon_cache(config, &backup_dir)?;
    }

    if config.backup_sends {
        info!("Backing up sends");
        backup_sends(config, &backup_dir)?;
    }

    // match (
    //     config.backup_db,
    //     config.backup_attachments,
    //     config.backup_config,
    //     config.backup_rsa_keys,
    //     config.backup_icon_cache,
    //     config.backup_sends,
    // ) {
    //     (true, true, true, true, true, true) => {
    //         info!("Backup configuration is set to backup everything");
    //         backup_database(&config, &backup_dir)?;
    //         backup_attachments(&config, &backup_dir)?;
    //         backup_config_file(&config, &backup_dir)?;
    //         backup_rsa_keys(&config, &backup_dir)?;
    //         backup_icon_cache(&config, &backup_dir)?;
    //         backup_sends(&config, &backup_dir)?;
    //     }
    //     (true, true, true, true, true, false) => {
    //         info!("Backup configuration is set to backup everything except sends");
    //         backup_database(&config, &backup_dir)?;
    //         backup_attachments(&config, &backup_dir)?;
    //         backup_config_file(&config, &backup_dir)?;
    //         backup_rsa_keys(&config, &backup_dir)?;
    //         backup_icon_cache(&config, &backup_dir)?;
    //     }
    //     (true, true, true, true, false, false) => {
    //         info!("Backup configuration is set to backup everything except sends and icon cache");
    //         backup_database(&config, &backup_dir)?;
    //         backup_attachments(&config, &backup_dir)?;
    //         backup_config_file(&config, &backup_dir)?;
    //         backup_rsa_keys(&config, &backup_dir)?;
    //     }
    //     (true, true, true, false, false, false) => {
    //         info!("Backup configuration is set to backup everything except sends, icon cache, and RSA keys");
    //         backup_database(&config, &backup_dir)?;
    //         backup_attachments(&config, &backup_dir)?;
    //         backup_config_file(&config, &backup_dir)?;
    //     }
    //     (true, true, false, false, false, false) => {
    //         info!("Backup configuration is set to backup database and attachments");
    //         backup_database(&config, &backup_dir)?;
    //         backup_attachments(&config, &backup_dir)?;
    //     }
    //     (true, false, false, false, false, false) => {
    //         info!("Backup configuration is set to backup database");
    //         backup_database(&config, &backup_dir)?;
    //     }
    //     (false, false, false, false, false, false) => {
    //         warn!("Backup configuration is set to backup nothing");
    //     }
    //     _ => {
    //         error!("Invalid backup options selected");
    //     }
    // };

    Ok(())
}

fn check_dirs_exist(config: &Config) -> Result<(), String> {
    if config.source_dir.is_empty() {
        error!("Source directory does not exist");
        return Err("Source directory does not exist".to_string());
    }

    if config.backup_dir.is_empty() {
        error!("Backup directory does not exist");
        return Err("Backup directory does not exist".to_string());
    }

    Ok(())
}
