use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::eyre::eyre;
use tracing::{debug, error, info};

use crate::utils::Config;

pub fn backup_rsa_keys(config: &Config, backup_dir: &Path) -> Result<(), color_eyre::Report> {
    info!("Backing up RSA keys");

    let source_dir = PathBuf::from(&config.source_dir);

    let read_result = source_dir.read_dir();

    let keys = read_result
        .expect("Could not read source directory")
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                if path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with("rsa_key")
                {
                    Some(path.to_path_buf())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let key_strings: Vec<OsString> = keys.iter().map(|p| p.as_os_str().to_os_string()).collect();

    debug!("Found {} RSA keys", key_strings.len());

    let backup_files = backup_dir;

    let output = Command::new("rsync")
        .arg("-a")
        .args(&key_strings)
        .arg(backup_files)
        .output()
        .map_err(|e| eyre!("Failed to execute rsync: {}", e))?;

    if output.status.success() {
        info!("RSA keys backed up");
        Ok(())
    } else {
        error!("Failed to backup RSA keys. rsync output: {:?}", output);
        Err(eyre!(
            "Failed to backup RSA keys. rsync output: {:?}",
            output
        ))
    }
}
