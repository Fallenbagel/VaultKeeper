use color_eyre::{eyre::eyre, Section, SectionExt};
use tracing::{error, info};

use super::{
    attachments::backup_attachments, config_file::backup_config_file, database::backup_database,
    icon_cache::backup_icon_cache, rsa_keys::backup_rsa_keys, sends::backup_sends,
    utils::create_backup_dir,
};

use crate::utils::Config;

pub fn perform_backups(config: &Config) -> Result<(), color_eyre::Report> {
    if check_dirs_exist(config).is_err() {
        return Err(eyre!("Source and/or backup directories are empty"))
            .with_warning(|| {
                "Please make sure you have set the source and backup directories correctly in config.json"
            })
            .with_section(|| {format!("source_dir: \"{}\"\nbackup_dir: \"{}\"", config.source_dir, config.backup_dir).header("config.json")});
    }

    let backup_dir = create_backup_dir(config)?;

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
