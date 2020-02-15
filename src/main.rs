extern crate raster;
extern crate num_cpus;
extern crate palette;
extern crate serde;

mod mandelbrot;
mod utils;

use mandelbrot::Mandelbrot;
use mandelbrot::config::ImageConfig;

use utils::*;

use std::time::SystemTime;
use std::process::Command;

fn main() {

    let config = ImageConfig::read_form_file_or_default("config.json");

    let elements_count = (config.pixel_range().0 * config.pixel_range().1) as u64;

    println!(
        "Minimum RAM usage for resolution {}x{}: {}",
         config.pixel_range().0, 
         config.pixel_range().1,
         bytes_string(calc_ram_req::<u16>(elements_count))
    );

    pause();

    let mandelbrot = Mandelbrot::new(config.max_iterations(), config.pixel_range());

    let whole_start = SystemTime::now();

    let frame = mandelbrot.get_iterations_frame((config.re_range(), config.im_range()), config.threads());

    println!("Elapsed time: {}", format_time(whole_start.elapsed().unwrap().as_millis()));

    let image = frame.to_image(&config.max_iterations());

    raster::save(&image, "img.png");
    
    Command::new("powershell")
        .arg("start img.png")
        .output()
        .unwrap();
}
