extern crate raster;
extern crate num_cpus;
extern crate palette;
extern crate serde;
extern crate clap;

pub mod mandelbrot;
pub mod utils;

use mandelbrot::Mandelbrot;
use mandelbrot::config::ImageConfig;
use mandelbrot::trans::{ImageWriter, FramePart};

use utils::{worker::Worker, loader::ConsoleLoader};

use std::time::SystemTime;
use std::process::Command;
use clap::{Arg, App};

fn main() {

    let matches = App::new("Mandelbrot")
                    .version("1.0")
                    .author("github.com/lupuuss")
                    .about("Generates mandelbrot and julia sets!")
                    .arg(Arg::with_name("real")
                            .takes_value(true)
                            .short("r")
                            .long("real")
                            .validator(utils::numeric_validator)
                            .required(false))
                    .arg(Arg::with_name("imag")
                            .short("i")
                            .takes_value(true)
                            .long("imag")
                            .validator(utils::numeric_validator)
                            .required(false))
                    .get_matches();

    print!("Loading config... ");

    let config = ImageConfig::read_form_file_or_default("config.json");

    println!("Done!");

    let elements_count = (config.pixel_range().0 * config.pixel_range().1) as u64;

    println!(
        "Minimum RAM usage for resolution {}x{}: {}",
         config.pixel_range().0, 
         config.pixel_range().1,
         utils::bytes_string(utils::calc_ram_req::<u16>(elements_count))
    );

    let julia_c = utils::parse_julia_c(&matches);

    match julia_c { Some(c) => println!("Picked julia c: {:?}", c), None => () }

    utils::pause();

    let timer = SystemTime::now();
   
    let mandelbrot = Mandelbrot::new(config.max_iterations(), config.pixel_range());
    let mut worker: Worker<FramePart> = Worker::new(config.threads(), false);

    let parts = mandelbrot.generate_frame_julia_on_worker(
        (config.re_range(), config.im_range()), config.threads(), &mut worker, julia_c
    );

    let mut image_writer = ImageWriter::new(config.pixel_range());

    let mut loader = ConsoleLoader::new(50);

    for i in 0..parts {

        let result = worker.output_receiver().recv().unwrap();
        
        loader.update(((i as f64 / parts as f64) * 100.0).round());
        loader.print_progress();

        image_writer.write_part(result, config.max_iterations());
    }

    loader.finish();

    println!("Elapsed time: {}", utils::format_time(timer.elapsed().unwrap().as_millis()));

    let image = image_writer.to_image();

    raster::save(&image, "img.png").unwrap();
    
    Command::new("powershell")
        .arg("start img.png")
        .output()
        .unwrap();
}
