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
        let mut s = Config::new();
        s.merge(File::with_name("config/default"))?;
        s.merge(File::with_name("config/clients"))?;

        s.try_into()
    }
}
