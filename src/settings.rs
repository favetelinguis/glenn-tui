use config::{Config, ConfigError, Environment, File};
use failure::ResultExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Role {
    arn: String,
    region: String,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    key: String,
    secret: String,
    roles: Vec<Role>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    clients: Vec<Account>,
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
