use color_eyre::eyre::Context;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{create_dir_all, File},
    io::{BufReader, Write},
    path::PathBuf,
};
use tracing::{debug, info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub source_dir: String,
    pub backup_dir: String,
    pub backup_count: u8,
    pub backup_db: bool,
    pub backup_attachments: bool,
    pub backup_config: bool,
    pub backup_rsa_keys: bool,
    pub backup_icon_cache: bool,
    pub backup_sends: bool,
}

impl Config {
    fn new() -> Self {
        Config {
            source_dir: String::from(""),
            backup_dir: String::from(""),
            backup_count: 5,
            backup_db: true,
            backup_attachments: true,
            backup_config: true,
            backup_rsa_keys: true,
            backup_icon_cache: true,
            backup_sends: true,
        }
    }
}

pub fn read_config(args: &Option<PathBuf>) -> Result<(Config, PathBuf), color_eyre::Report> {
    let config_path = match args {
        Some(path) => path.join("config.json"),
        None => env::current_dir().unwrap().join("config.json"),
    };

    match read_config_from_path(&config_path) {
        Ok(config) => {
            info!("Config file found. Using values from config file");
            Ok((config, config_path))
        }
        Err(_) => {
            warn!("No config file found, creating one with default values");
            generate_config(&config_path)
        }
    }
}

pub fn read_config_from_path(config_path: &PathBuf) -> Result<Config, color_eyre::Report> {
    println!("Reading config from: {:?}", config_path);
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    debug!("Config: {:?}", config);
    Ok(config)
}

pub fn generate_config(config_path: &PathBuf) -> Result<(Config, PathBuf), color_eyre::Report> {
    create_dir_all(config_path.parent().unwrap()).wrap_err_with(|| {
        format!(
            "Couldn't create config directory at {}",
            config_path.parent().unwrap().to_str().unwrap()
        )
    })?;
    let config = Config::new();
    write_config(&config, config_path)?;
    Ok((config, config_path.to_path_buf()))
}

fn write_config(config: &Config, config_path: &PathBuf) -> Result<(), color_eyre::Report> {
    let json = serde_json::to_string_pretty(config)?;
    let mut file = File::create(config_path).wrap_err_with(|| {
        format!(
            "Couldn't create config file at {}",
            config_path.to_str().unwrap()
        )
    })?;

    if let Err(e) = file.write_all(json.as_bytes()) {
        panic!("Couldn't write to file: {}", e);
    }
    Ok(())
}
