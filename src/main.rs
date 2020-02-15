extern crate raster;
extern crate num_cpus;
extern crate palette;
extern crate serde;

mod mandelbrot;

use mandelbrot::Mandelbrot;
use mandelbrot::config::ImageConfig;
use std::time::SystemTime;
use std::process::Command;

fn format_time(miliseconds: u128) -> String {

    let in_miliseconds = miliseconds;

    let miliseconds = miliseconds % 1000;
    let in_seconds = in_miliseconds / 1000;

    let seconds = in_seconds % 60;
    let minutes = in_seconds / 60;

    let mut time_str = String::new();

    if minutes > 0 {
        time_str += &minutes.to_string();
        time_str += " min ";
    }

    if seconds > 0 {
        time_str += &seconds.to_string();
        time_str += " s ";
    }

    if miliseconds > 0 {
        time_str += &miliseconds.to_string();
        time_str += " ms"
    }

    return time_str;
}

fn main() {

    let config = ImageConfig::read_form_file_or_default("config.json");
    
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
