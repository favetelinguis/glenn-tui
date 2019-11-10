use config::{Config, ConfigError, Environment, File};
use failure::ResultExt;
use serde::{Deserialize, Serialize};

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
        let mut config_path = dirs::home_dir().unwrap();
        config_path.push(".config");
        config_path.push("glenn");
        config_path.push("clients");

        let mut s = Config::new();
        s.merge(File::with_name("config/default").required(true))?;
        s.merge(File::from(config_path).required(false))?;

        s.try_into()
    }
}
