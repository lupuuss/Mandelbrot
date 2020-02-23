
use std::io::{stdout, Stdout};
use std::io::prelude::*;

pub struct ConsoleLoader {
    current_percentage: f64,
    std_out: Stdout,
    last_write_len: usize,
    particles: usize
}

impl ConsoleLoader {
    pub fn new(particles: usize) -> ConsoleLoader {
        ConsoleLoader {
            current_percentage: 0.0,
            std_out: stdout(),
            last_write_len: 0,
            particles: particles
        }
    }

    pub fn update(&mut self, new_percentage: f64) {
        self.current_percentage = new_percentage;
    }

    pub fn print_progress(&mut self) {

        let mut remove_str = String::new();

        for _ in 0..self.last_write_len {
            remove_str.push('\x08');
            remove_str.push(' ');
            remove_str.push('\x08');
        }

        self.std_out.write(remove_str.as_bytes()).unwrap();

        let loader_str = self.create_progress_str();
        self.last_write_len = loader_str.chars().count();

        self.std_out.write(loader_str.as_bytes()).unwrap();
        self.std_out.flush().unwrap();
    }

    pub fn finish(mut self) {
        self.update(100.0);
        self.print_progress();
        self.std_out.write("\n".as_bytes()).unwrap();
        self.std_out.flush().unwrap();
    }

    fn create_progress_str(&self) -> String {
        let mut progress_str = String::from("[");
        let done_particles = ((self.current_percentage / 100.0) * self.particles as f64).round() as usize;

        for _ in 0..done_particles {
            progress_str.push('=');
        }

        for _ in 0..self.particles - done_particles {
            progress_str.push(' ');
        }

        progress_str.push_str("] ");

        progress_str.push_str(&self.current_percentage.to_string());
        progress_str.push('%');

        progress_str
    }
}