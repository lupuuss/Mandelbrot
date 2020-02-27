pub mod worker;
mod cli;
mod gui;

use super::fractal::config::Config;
use super::fractal::{FractalGenerator, Fractal};
use super::fractal::math::ComplexF64;
use cli::CliRunner;
use gui::GuiRunner;

use std::sync::{Arc, RwLock};

pub enum Mode {
    CliStatic,
    GuiDynamic
}

impl Mode {
    pub fn new_runner(runner_type: Mode, config: Config, julia_c: Option<(f64, f64)>) -> Box<dyn ModeRunner> {

        let generator = match julia_c {
            Some(c) => Fractal::JuliaSet(
                config.pixel_range(), config.max_iterations(), ComplexF64{ re: c.0, im: c.1 }
            ),
            None => Fractal::Mandelbrot(config.pixel_range(), config.max_iterations())
        };

        match runner_type {
            Mode::CliStatic => Box::new(CliRunner::new(config, generator)),
            Mode::GuiDynamic => Box::new(GuiRunner::new(config, generator))
        }
    }
}

pub struct BaseRunner {
    config: Config,
    generator: Arc<RwLock<dyn FractalGenerator + Send + Sync>>
}

impl BaseRunner {

    pub fn new(config: Config, generator: Fractal) -> Self {
        BaseRunner {
            config: config,
            generator: Fractal::new_thread_safe_generator(generator)
        }
    }
    
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn generator(&self) -> Arc<RwLock<dyn FractalGenerator + Send + Sync>> {
        Arc::clone(&self.generator)
    }
}

pub trait ModeRunner {

    fn start(&mut self);
}