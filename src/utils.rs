use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use std::{
    env,
    fs::File,
    io::{BufReader, Error, Write},
    path::PathBuf,
};

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

pub fn read_config(args: Option<PathBuf>) -> Result<Config, SerdeJsonError> {
    if let Some(config_path) = args {
        let config_file_path = config_path.join("config.json");
        match read_config_from_path(&config_file_path) {
            Ok(config) => {
                info!("Config file found. Using values from config file");
                Ok(config)
            }
            Err(_) => {
                warn!("No config file found, creating one with default values");
                let config_file_path = config_path.join("config.json");
                generate_config(&config_file_path)
            }
        }
    } else {
        let current_dir = env::current_dir().unwrap();
        let config_path = current_dir.join("config.json");

        match read_config_from_path(&config_path) {
            Ok(config) => {
                info!("Config file found. Using values from config file");
                Ok(config)
            }
            Err(_) => {
                warn!("No config file found, creating one with default values");
                generate_config(&config_path)
            }
        }
    }
}

fn read_config_from_path(config_path: &PathBuf) -> Result<Config, Error> {
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    debug!("Config: {:?}", config);
    Ok(config)
}

pub fn generate_config(config_path: &PathBuf) -> Result<Config, SerdeJsonError> {
    let config = Config::new();
    let _ = write_config(&config, config_path);
    Ok(config)
}

fn write_config(config: &Config, config_path: &PathBuf) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(config)?;
    let mut file = File::create(config_path).expect("Could not create config file");

    if let Err(e) = file.write_all(json.as_bytes()) {
        panic!("Couldn't write to file: {}", e);
    }
    Ok(())
}
