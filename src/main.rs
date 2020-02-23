extern crate raster;
extern crate num_cpus;
extern crate palette;
extern crate serde;

pub mod mandelbrot;
pub mod utils;

use mandelbrot::Mandelbrot;
use mandelbrot::config::ImageConfig;
use mandelbrot::trans::{ImageWriter, FramePart};

use utils::{*, worker::Worker};

use std::time::SystemTime;
use std::process::Command;

fn main() {

    print!("Loading config... ");

    let config = ImageConfig::read_form_file_or_default("config.json");

    println!("Done!");

    let elements_count = (config.pixel_range().0 * config.pixel_range().1) as u64;

    println!(
        "Minimum RAM usage for resolution {}x{}: {}",
         config.pixel_range().0, 
         config.pixel_range().1,
         bytes_string(calc_ram_req::<u16>(elements_count))
    );

    pause();

    let timer = SystemTime::now();
   
    let mandelbrot = Mandelbrot::new(config.max_iterations(), config.pixel_range());
    let mut worker: Worker<FramePart> = Worker::new(config.threads(), false);

    let parts = mandelbrot.generate_frame_on_worker(
        (config.re_range(), config.im_range()), config.threads(), &mut worker
    );

    let mut image_writer = ImageWriter::new(config.pixel_range());

    for _ in 0..parts {

        let result = worker.output_receiver().recv().unwrap();
        println!("{:?} part done!", result.range());

        image_writer.write_part(result, config.max_iterations());
    }

    println!("Elapsed time: {}", format_time(timer.elapsed().unwrap().as_millis()));

    let image = image_writer.to_image();

    raster::save(&image, "img.png").unwrap();
    
    Command::new("powershell")
        .arg("start img.png")
        .output()
        .unwrap();
}
