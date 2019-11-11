#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod clients;
mod demo;
mod settings;
mod util;

use crate::clients::Clients;
use crate::demo::{ui, App};
use crate::settings::Settings;
use crate::util::event::{Config, Event, Events};

use exitfailure::ExitFailure;
use failure::ResultExt;
use log::{debug, info};
use std::io;
use std::time::Duration;
use structopt::StructOpt;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;
use rusoto_ssm::{Ssm, GetParametersByPathRequest};

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "tick-rate", default_value = "250")]
    tick_rate: u64,
    #[structopt(long = "log")]
    log: bool,
}

fn main() -> Result<(), ExitFailure> {
    let cli = Cli::from_args();
    stderrlog::new().quiet(!cli.log).verbosity(4).init()?;

    info!("Starting glenn-rs");
    let settings = &Settings::new()?;
    debug!("Config {:?}", settings);

    let clients = &Clients::new(settings);
    let available_clients = clients.available_clients();
    for c in clients.create_ssm_clients(available_clients) {
        let mut request = GetParametersByPathRequest::default();
        request.with_decryption = Some(true);
        request.recursive = Some(true);
        request.path = "/dpap/".to_string();
        let response = c.get_parameters_by_path(request).sync().unwrap();
        println!("Parameters: {:?}: ", response.parameters.unwrap());
    }

    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(cli.tick_rate),
        ..Config::default()
    });

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new("GLENN alpha version");
    loop {
        ui::draw(&mut terminal, &app)?;
        match events.next()? {
            Event::Input(key) => match key {
                Key::Char(c) => {
                    app.on_key(c);
                }
                Key::Up => {
                    app.on_up();
                }
                Key::Down => {
                    app.on_down();
                }
                Key::Left => {
                    app.on_left();
                }
                Key::Right => {
                    app.on_right();
                }
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
