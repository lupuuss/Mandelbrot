use super::ModeRunner;
use super::Config;

pub struct GuiRunner {}

impl GuiRunner {
    pub fn new() -> Self {
        GuiRunner {}
    }
}

impl ModeRunner for GuiRunner {

    fn start(&mut self, _config: &Config) {
        panic!("Not implemented yet!");
    }
}