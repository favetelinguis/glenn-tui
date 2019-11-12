use crate::util::TabsState;
use crate::clients::Clients;

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub clients: Clients<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, clients: Clients<'a>) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Parameter Store", "ECS Service"]),
            clients,
        }
    }

    pub fn on_up(&mut self) {}

    pub fn on_down(&mut self) {}

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {}
}
