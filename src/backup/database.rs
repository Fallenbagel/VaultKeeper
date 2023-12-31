use std::{path::Path, process::Command};

use color_eyre::eyre::eyre;
use tracing::{error, info};

use crate::utils::Config;

pub fn backup_database(config: &Config, backup_dir: &Path) -> Result<(), color_eyre::Report> {
    info!("Backing up database");

    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");

    let backup_file = backup_dir.join(format!("db-{}.sqlite3", timestamp));

    let output = Command::new("sqlite3")
        .arg("db.sqlite3")
        .arg(format!(".backup {:?}", backup_file))
        .current_dir(&config.source_dir)
        .output()
        .map_err(|e| eyre!("Failed to execute sqlite3: {}", e))?;

    if output.status.success() {
        info!("Database backed up");
        Ok(())
    } else {
        error!("Failed to backup database. sqlite3 output: {:?}", output);
        Err(eyre!(
            "Failed to backup database. sqlite3 output: {:?}",
            output
        ))
    }
}
