use std::{
    env::current_dir,
    fmt::{Display, Formatter},
    fs::File,
    io::Write,
    path::PathBuf,
};

use color_eyre::eyre::eyre;
use color_eyre::Section;
use regex::Regex;
use tracing::{debug, warn};

#[derive(Debug, Clone)]
pub struct ActivationTime {
    daily: bool,
    time: Option<String>,
}

impl From<String> for ActivationTime {
    fn from(value: String) -> Self {
        match parse_schedule_args(value) {
            Ok(activation_time) => activation_time,
            Err(_) => {
                warn!("Activation time is an invalid format");
                ActivationTime::new(false, None)
            }
        }
    }
}

impl ActivationTime {
    fn new(daily: bool, time: Option<&str>) -> Self {
        let time = time.map(|t| t.to_string());
        // let time = time.as_deref();
        ActivationTime { daily, time }
    }
}

impl Display for ActivationTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.daily {
            write!(f, "daily")
        } else if let Some(time) = &self.time {
            write!(f, "{}", time)
        } else {
            panic!("ActivationTime is invalid")
        }
    }
}

fn parse_schedule_args(args: String) -> Result<ActivationTime, color_eyre::Report> {
    let daily = args.eq_ignore_ascii_case("daily");
    let time = if !daily {
        let re = Regex::new(r"(\d{2}:\d{2})?").unwrap();
        if let Some(captures) = re.captures(&args) {
            if let Some(time_str) = captures.get(1) {
                Some(time_str.as_str().to_string())
            } else {
                return Err(eyre!("Activation time is an invalid format"));
            }
        } else {
            None
        }
    } else {
        None
    };

    let activation_time = ActivationTime::new(daily, time.as_deref());
    debug!("{:#?}", activation_time);

    Ok(activation_time)
}

pub fn create_systemd_timer(
    activation_time: &ActivationTime,
    manual: &Option<bool>,
) -> Result<(), color_eyre::Report> {
    let mut path = "/etc/systemd/system".to_string();

    if manual.unwrap_or(false) {
        path = current_dir().unwrap().to_str().unwrap().to_string();
    }

    let mut file =
        File::create(format!("{}/{}.timer", path, env!("CARGO_PKG_NAME"))).map_err(|e| {
            eyre!("Could not create systemd timer file: {}", e)
                .with_warning(|| {
                    format!(
                        "Please make sure you have the correct permissions to create files in {}",
                        path
                    )
                })
                .with_suggestion(|| "Try running with sudo")
        })?;
    let timer_contents = format!(
        "[Unit]\nDescription={} timer to run backup\n\n[Timer]\nOnCalendar=*-*-* {}\nUnit={}\nPersistent=true\n\n[Install]\nWantedBy=timers.target",
        env!("CARGO_PKG_NAME"),
        activation_time,
        env!("CARGO_PKG_NAME")
    );

    file.write_all(timer_contents.as_bytes())?;

    Ok(())
}

pub fn create_systemd_service(
    manual: &Option<bool>,
    config: &PathBuf,
) -> Result<(), color_eyre::Report> {
    let mut path = "/etc/systemd/system".to_string();
    let config_path = config;

    if manual.unwrap_or(false) {
        path = current_dir().unwrap().to_str().unwrap().to_string();
    } else {
        check_binary_exists()?;
        check_config_exists()?;
    }

    let mut file =
        File::create(format!("{}/{}.service", path, env!("CARGO_PKG_NAME"))).map_err(|e| {
            eyre!("Could not create systemd service file: {}", e)
                .with_warning(|| {
                    format!(
                        "Please make sure you have the correct permissions to create files in {}",
                        path
                    )
                })
                .with_suggestion(|| "Try running with sudo")
        })?;
    let service_contents = format!(
        "[Unit]\nDescription={} service to run backup\n\n[Service]\nType=oneshot\nExecStart=/usr/bin/{} backup --config {:?}\n\n[Install]\nWantedBy=multi-user.target",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_NAME"),
        config_path
    );

    file.write_all(service_contents.as_bytes())?;

    Ok(())
}

fn check_config_exists() -> Result<(), color_eyre::Report> {
    let config_path = "/etc/vaultkeeper/config.json";
    if !std::path::Path::new(config_path).exists() {
        return Err(eyre!("Config file does not exist")).with_suggestion(|| {
            "Generate a config file with `vaultkeeper generate-config --config /etc/vaultkeeper/config.json`"
        });
    }

    Ok(())
}

fn check_binary_exists() -> Result<(), color_eyre::Report> {
    let binary_name = env!("CARGO_PKG_NAME");
    let paths = std::env::var("PATH").unwrap();
    for path in paths.split(':') {
        let path = PathBuf::from(path);
        let full_path = path.join(binary_name);
        if full_path.exists() {
            return Ok(());
        }
    }

    Err(eyre!("Binary does not exist"))
        .with_suggestion(|| "Install the binary with `--binary true`")
        .with_section(|| {
            format!(
                "Binary not found in PATH. Make sure {} is in your PATH",
                binary_name
            )
        })
}
