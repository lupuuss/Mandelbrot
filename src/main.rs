extern crate raster;
extern crate num_cpus;
extern crate palette;
extern crate serde;

mod mandelbrot;

use mandelbrot::Mandelbrot;
use mandelbrot::config::ImageConfig;
use std::time::SystemTime;
use std::process::Command;

fn main() {

    let config = ImageConfig::read_form_file_or_default("config.json");
    
    let mandelbrot = Mandelbrot::new(config.max_iterations(), config.pixel_range());

    let whole_start = SystemTime::now();

    let frame = mandelbrot.get_iterations_frame((config.re_range(), config.im_range()), config.threads());

    println!("Elapsed time: {} ms", whole_start.elapsed().unwrap().as_millis());

    let image = frame.to_image(&config.max_iterations());

    raster::save(&image, "img.png");

    Command::new("powershell")
    .arg("start img.png")
    .output()
    .unwrap();
}
