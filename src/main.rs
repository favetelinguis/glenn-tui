#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod clients;
mod settings;

use clients::Clients;
use exitfailure::ExitFailure;
use failure::ResultExt;
use log::{debug, info};
use settings::Settings;

fn main() -> Result<(), ExitFailure> {
    env_logger::init();
    info!("Starting glenn-rs");
    let settings = &Settings::new()?;
    let clients = &Clients::new(settings);

    debug!("Config {:?}", settings);

    Ok(())
}
