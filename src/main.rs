#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod settings;

use exitfailure::ExitFailure;
use failure::ResultExt;
use log::{debug, info};
use settings::Settings;

fn main() -> Result<(), ExitFailure> {
    env_logger::init();
    info!("Starting glenn-rs");
    let settings = &Settings::new()?;

    debug!("Config {:?}", settings);

    Ok(())
}
