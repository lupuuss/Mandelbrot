extern crate raster;

mod model;

use model::mandelbrot::*;
use raster::Image;
use raster::Color;
use std::time::{SystemTime, Duration};

fn determine_color(iterations: i32, max_iterations: i32) -> Color {

    return if iterations >= max_iterations {
        Color::black()
    } else {
        Color::white()
    }
}

fn main() {

    let mandelbrot = Mandelbrot::new(1000, ((-2.0, 0.5), (-1.0, 1.0)), (1250, 1000));
    let (width, height) = mandelbrot.get_pixel_range();

    let particles = mandelbrot.between_pixels();
    let mut image = Image::blank(width, height);

    let mut x = mandelbrot.get_start_point();

    println!("Particles: {:?}", particles);

    let whole_start = SystemTime::now();

    let real_range_start = mandelbrot.get_real_range().0;

    for i in 0..height {

        for j in 0..width {

            image.set_pixel(
                j, i, 
                determine_color(mandelbrot.convergence_iterations(&x), mandelbrot.get_max_iterations())
            ).unwrap();

            x.0 += particles.0;
        }

        x.0 = real_range_start;

        x.1 -= particles.1;
    }

    println!("Elapsed time: {} ms", whole_start.elapsed().unwrap().as_millis());

    raster::save(&image, "img.png");

}
