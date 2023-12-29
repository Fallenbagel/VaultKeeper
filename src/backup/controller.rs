use std::{fs, path::PathBuf};

use color_eyre::{eyre::eyre, Section, SectionExt};
use tracing::debug;

use super::{
    attachments::backup_attachments, config_file::backup_config_file, database::backup_database,
    icon_cache::backup_icon_cache, rsa_keys::backup_rsa_keys, sends::backup_sends,
    utils::create_backup_dir,
};

use crate::utils::Config;

#[derive(Debug)]
struct EmptyCharacterPos(u32, u32);

pub fn perform_backups(config: &Config, config_path: &PathBuf) -> Result<(), color_eyre::Report> {
    // if check_dirs_exist(config).is_err() {
    //     return Err(eyre!("Source and/or backup directories are empty"))
    //         .with_warning(|| { format!("Please make sure you have set the source and backup directories correctly in {:?}", config_path.join("config.json")) })
    //         .with_section(|| {
    //             format!(
    //                 "source_dir: \"{}\"\nbackup_dir: \"{}\"",
    //                 config.source_dir, config.backup_dir
    //             )
    //             .header("config.json")
    //         });
    // }

    check_dirs_exist(config, config_path)?;

    let backup_dir = create_backup_dir(config)?;

    if config.backup_db {
        debug!("Backing up database");
        backup_database(config, &backup_dir)?;
    }

    if config.backup_attachments {
        debug!("Backing up attachments");
        backup_attachments(config, &backup_dir)?;
    }

    if config.backup_config {
        debug!("Backing up config file");
        backup_config_file(config, &backup_dir)?;
    }

    if config.backup_rsa_keys {
        debug!("Backing up RSA keys");
        backup_rsa_keys(config, &backup_dir)?;
    }

    if config.backup_icon_cache {
        debug!("Backing up icon cache");
        backup_icon_cache(config, &backup_dir)?;
    }

    if config.backup_sends {
        debug!("Backing up sends");
        backup_sends(config, &backup_dir)?;
    }

    Ok(())
}

fn check_dirs_exist(config: &Config, config_path: &PathBuf) -> Result<(), color_eyre::Report> {
    if config.source_dir.is_empty() {
        return Err(eyre!("Source directory is empty")
            .with_suggestion(|| {
                {
                    format!("Set the source directories correctly in {:?}", config_path)
                }
            })
            .with_section(|| {
                format!("source_dir: {:#?}", config.source_dir).header(format!(
                    "\u{1b}[35m{}\u{1b}[0m:\u{1b}[35m2\u{1b}[0m:\u{1b}[35m18\u{1b}[0m",
                    &config_path.display()
                ))
            }));
    }

    if config.backup_dir.is_empty() {
        return Err(eyre!("Backup directory does not exist")
            .with_suggestion(|| {
                {
                    format!(
                        "Set the backup directories correctly in {:?}",
                        config_path.join("config.json")
                    )
                }
            })
            .with_section(|| {
                format!("backup_dir: {:#?}", config.source_dir)
                    .header(format!("{:#?}", config_path.join("config.json")))
            }));
    }

    Ok(())
}

// fn line_number_and_column_detect(
//     config_path: &PathBuf,
// ) -> Result<EmptyCharacterPos, color_eyre::Report> {
//     let config_json = fs::read_to_string(config_path)?;

//     let line = 0;
//     let col = 0;

//     if let Some(idx) = config_json.find(':') {
//         if let Some(idx2) = config_json[idx + 2..].find("\"\"") {
//             let line = config_json[..idx].matches('\n').count() + 1;
//             let col = idx + idx2 + 2;

//             println!("Empty source_dir at line {} column {}", line, col);
//         }
//     }

//     Ok(EmptyCharacterPos(line, col))
// }
