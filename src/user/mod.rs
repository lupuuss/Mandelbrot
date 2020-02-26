pub mod worker;
mod cli;
mod gui;

use super::fractal::config::Config;
use cli::CliRunner;
use gui::GuiRunner;

pub enum Mode {
    CliStatic(Option<(f64, f64)>),
    GuiDynamic
}

impl Mode {
    pub fn new_runner(runner_type: Mode) -> Box<dyn ModeRunner> {
        match runner_type {
            Mode::CliStatic(julia_c) => Box::new(CliRunner::new(julia_c)),
            Mode::GuiDynamic => Box::new(GuiRunner::new())
        }
    }
}

pub trait ModeRunner {

    fn start(&mut self, config: &Config);
}