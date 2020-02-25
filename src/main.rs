extern crate raster;
extern crate num_cpus;
extern crate palette;
extern crate serde;
extern crate clap;

pub mod fractal;
pub mod utils;

use utils::{worker::Worker, loader::ConsoleLoader};
use fractal::Fractal;
use fractal::trans::{FramePart, ImageWriter};
use fractal::config::ImageConfig;
use fractal::math::ComplexF64;

use std::time::{SystemTime, UNIX_EPOCH};
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

    let fractal_type =  match julia_c { 
        Some(c) =>  { 
            println!("Picked julia c: {:?}", c);
            Fractal::JuliaSet(config.pixel_range(), config.max_iterations(), ComplexF64 { re: c.0, im: c.1 }) 
        }, 
        None => Fractal::Mandelbrot(config.pixel_range(), config.max_iterations()) 
    }; 

    utils::pause();

    let timer = SystemTime::now();
   
    let generator = Fractal::new_thread_safe_generator(fractal_type);
    let mut worker: Worker<FramePart> = Worker::new(config.threads(), false);

    let parts = Fractal::generate_frame_on_worker(
        generator, 
        config.complex_range(),
        config.threads() * config.thread_split(),
        &mut worker
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

    let mut now_png = String::from(
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string()
    );
    now_png.push_str(".png");

    raster::save(&image, &now_png).unwrap();
    
    let mut start_png = String::from("start ");
    start_png.push_str(&now_png);

    Command::new("powershell")
        .arg(start_png)
        .output()
        .unwrap();
}
