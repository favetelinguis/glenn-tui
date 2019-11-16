#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod app;
mod banner;
mod clients;
mod settings;
mod ui;
mod util;

use crate::app::App;
use crate::banner::BANNER;
use crate::clients::Clients;
use crate::settings::Settings;
use crate::util::event::{Event, Events};

use exitfailure::ExitFailure;
use failure::ResultExt;
use log::{debug, info};
use rusoto_ssm::{GetParametersByPathRequest, Ssm};
use std::io;
use std::time::Duration;
use clap::App as ClapApp;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

fn main() -> Result<(), ExitFailure> {
    ClapApp::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .usage("Press `?` while running the app to see keybindings")
        .before_help(BANNER)
        .after_help("All config should be placed in ~/.config/glenn/")
        .get_matches();

    // quite should come as a line arg
    stderrlog::new().quiet(false).verbosity(4).init()?;

    info!("Starting glenn-rs");
    let settings = &Settings::new()?;
    debug!("Config {:?}", settings);

    //    let available_clients = clients.available_clients();
    //    for c in clients.create_ssm_clients(available_clients) {
    //        let mut request = GetParametersByPathRequest::default();
    //        request.with_decryption = Some(true);
    //        request.recursive = Some(true);
    //        request.path = "/dpap/".to_string();
    //        let response = c.get_parameters_by_path(request).sync().unwrap();
    //        println!("Parameters: {:?}: ", response.parameters.unwrap());
    //    }

    // TODO i can set this Duration::from_millis(cli.tick_rate)
    let events = Events::new();

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let clients = Clients::new(settings);
    let mut app = App::new("GLENN alpha version", clients);

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
