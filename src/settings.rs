use config::{Config, ConfigError, Environment, File};
use failure::ResultExt;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

const FILE_NAME: &str = "clients";
const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "glenn";

#[derive(Debug, Deserialize)]
pub struct Role {
    pub name: String,
    pub arn: String,
    pub region: String,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub key: String,
    pub secret: String,
    pub roles: Vec<Role>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub clients: Vec<Account>,
    test: usize,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config_path = get_or_build_paths();

        let mut s = Config::new();
        s.merge(File::with_name("config/default").required(true))?;
        s.merge(File::from(config_path).required(false))?;

        s.try_into()
    }
}
fn get_or_build_paths() -> PathBuf {
    match dirs::home_dir() {
        Some(home) => {
            let path = Path::new(&home);
            let home_config_dir = path.join(CONFIG_DIR);
            let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

            if !home_config_dir.exists() {
                fs::create_dir(&home_config_dir).unwrap();
            }

            if !app_config_dir.exists() {
                fs::create_dir(&app_config_dir).unwrap();
            }

            let config_file_path = &app_config_dir.join(FILE_NAME);

            Ok(config_file_path.to_path_buf())
        }
        None => Err("No $HOME directory found for client config"),
    }.unwrap()
}
