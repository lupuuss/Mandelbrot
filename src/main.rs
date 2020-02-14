extern crate raster;

mod mandelbrot;
extern crate num_cpus;

use mandelbrot::*;
use std::time::SystemTime;


fn main() {

    let mandelbrot = Mandelbrot::new(1000, (2500, 2000));

    let whole_start = SystemTime::now();

    let frame = mandelbrot.get_iterations_frame(((-2.0, 0.5), (-1.0, 1.0)), 16);

    println!("Elapsed time: {} ms", whole_start.elapsed().unwrap().as_millis());

    let image = frame.to_image(&1000);

    raster::save(&image, "img.png");

}
