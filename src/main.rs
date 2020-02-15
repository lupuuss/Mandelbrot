extern crate raster;

mod mandelbrot;
extern crate num_cpus;
extern crate palette;

use mandelbrot::*;
use std::time::SystemTime;


fn main() {

    let max_iterations = 7000;

    let mandelbrot = Mandelbrot::new(max_iterations, (5000, 4000));

    let whole_start = SystemTime::now();

    let frame = mandelbrot.get_iterations_frame(((-2.0, 0.5), (-1.0, 1.0)), 16);

    println!("Elapsed time: {} ms", whole_start.elapsed().unwrap().as_millis());

    let image = frame.to_image(&max_iterations);

    raster::save(&image, "img.png");

}
