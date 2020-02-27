use super::ModeRunner;
use super::Config;
use super::worker::Worker;

use super::super::utils;

use super::super::fractal as fractal;

use fractal::Fractal;
use fractal::trans::FramePart;
use fractal::math::ComplexF64;
use fractal::trans::SurfaceWriter;

use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command;
use std::io::{stdout, stdin, Stdout};
use std::io::prelude::*;

pub struct CliRunner {
    julia_c: Option<(f64, f64)>
}

impl CliRunner {
    pub fn new(julia_c: Option<(f64, f64)>) -> Self {
        CliRunner {
            julia_c: julia_c
        }
    }
}

fn calc_ram_req<StoredType>(total_elements: u64) -> u64 {
    return 2 * calc_array_total_size::<StoredType>(total_elements);
}

fn calc_array_total_size<StoredType>(total_elements: u64) -> u64 {
    return std::mem::size_of::<StoredType>() as u64 * total_elements;
}

fn pause() {
    let mut stdin = stdin();
    let mut stdout = stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

impl ModeRunner for CliRunner {

    fn start(&mut self, config: &Config) {

        let elements_count = (config.pixel_range().0 * config.pixel_range().1) as u64;

        println!(
            "Minimum RAM usage for resolution {}x{}: {}",
             config.pixel_range().0, 
             config.pixel_range().1,
             utils::bytes_string(calc_ram_req::<u16>(elements_count))
        );
        
        let fractal_type =  match self.julia_c { 
            Some(c) =>  { 
                println!("Picked julia c: {:?}", c);
                Fractal::JuliaSet(config.pixel_range(), config.max_iterations(), ComplexF64 { re: c.0, im: c.1 }) 
            }, 
            None => Fractal::Mandelbrot(config.pixel_range(), config.max_iterations()) 
        }; 
    
        pause();
    
        let timer = SystemTime::now();
       
        let generator = Fractal::new_thread_safe_generator(fractal_type);
        let mut worker: Worker<FramePart> = Worker::new(config.threads(), false);
    
        let parts = Fractal::generate_frame_on_worker(
            generator, 
            config.complex_range(),
            config.threads() * config.thread_split(),
            &mut worker
        );
    
        let width = config.pixel_range().0 as u32;
        let height = config.pixel_range().1 as u32;

        let mut surface_writer = SurfaceWriter::new_blank(width, height);
    
        let mut loader = Loader::new(50);
    
        for i in 0..parts {
    
            let result = worker.output_receiver().recv().unwrap();
            
            loader.update(((i as f64 / parts as f64) * 100.0).round());
            loader.print_progress();
    
            surface_writer.write_part(result, config.max_iterations());
        }
    
        loader.finish();
    
        println!("Elapsed time: {}", utils::format_time(timer.elapsed().unwrap().as_millis()));
    
        let mut now_png = String::from(
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string()
        );
        now_png.push_str(".png");
    
        surface_writer.save_to_image(&now_png).unwrap();
        
        let mut start_png = String::from("start ");
        start_png.push_str(&now_png);
    
        Command::new("powershell")
            .arg(start_png)
            .output()
            .unwrap();
    }
}

pub struct Loader {
    current_percentage: f64,
    std_out: Stdout,
    last_write_len: usize,
    particles: usize
}

impl Loader {
    pub fn new(particles: usize) -> Loader {
        Loader {
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