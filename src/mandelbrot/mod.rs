pub mod trans;
pub mod config;

use std::vec::Vec;
use trans::FramePart;

use super::utils::worker::Worker;

pub struct Mandelbrot {
    max_iteations: u16,
    pixel_range: (usize, usize)
}

impl Mandelbrot {

    pub fn new(max_iteations: u16, pixel_range: (usize, usize)) -> Mandelbrot {
        Mandelbrot {
            max_iteations: max_iteations,
            pixel_range: pixel_range
        }
    }

    pub fn generate_frame_on_worker(
        &self,
        complex_range: ((f64, f64), (f64, f64)), 
        split_work: usize, 
        worker: &mut Worker<FramePart>
    ) -> usize {

        let particles = self.between_pixels(complex_range);
    
        let (re_range, im_range) = complex_range;

        let x = (re_range.0, im_range.1);
    
        let (width, height) = self.pixel_range;

        let part_size = height / split_work;
        let leftovers = height % split_work;

        let split_count = split_work + if leftovers != 0 { 1 } else { 0 };

        let max = self.max_iteations;

        for i in 0..split_work {

            worker.push(Box::new(move || -> FramePart {

                let range = (i * part_size, ((i + 1) * part_size));
                let result = Mandelbrot::get_frame_part(x, &max, range, width, particles);
                return FramePart::new(range, result);
            }));
        }

        if leftovers != 0 {

            worker.push(Box::new(move || -> FramePart {
                let tmp = split_work * part_size;
                let range = (tmp, tmp + leftovers);
                let result = Mandelbrot::get_frame_part(x, &max, range, width, particles);

                return FramePart::new(range, result);
            }));
        }

        return split_count;
    }

    fn between_pixels(&self, complex_range: ((f64, f64), (f64, f64))) -> (f64, f64) {

        let (real_range, imaginary_range) = complex_range;

        if real_range.0 >= real_range.1 {
            panic!("Invalid real range: {:?}", real_range);
        }
    
        if imaginary_range.0 >= imaginary_range.1 {
            panic!("Invalid imaginary range: {:?}", imaginary_range);
        }
    
        let real_range_len = real_range.1 - real_range.0;
        let imaginary_range_len = imaginary_range.1 - imaginary_range.0;
    
        (real_range_len as f64 / self.pixel_range.0 as f64, imaginary_range_len as f64 / self.pixel_range.1 as f64)
    
    }

    fn convergence_iterations(max_iteations: &u16, n: &(f64, f64)) -> u16 {
        
        let mut i = 0;
        let mut result = (0.0, 0.0);

        while result.0 * result.0 + result.1 * result.1 < 4.0 && i < *max_iteations {

            result = Mandelbrot::complex_mul(&result, &result);
            result = (result.0 + n.0, result.1 + n.1);
            i += 1;
        }

        return i;
    }

    fn get_frame_part(start: (f64, f64), 
                      max_iteations: &u16, 
                      lines: (usize, usize), 
                      width: usize, 
                      particles: (f64, f64)) -> Vec<u16> {

        let mut x = start;
        x.1 -= lines.0 as f64 * particles.1;

        let real_range_start = start.0;
    
        let mut frame_part: Vec<u16> = Vec::with_capacity((lines.1 - lines.0) * width);

        for _ in (lines.0)..(lines.1) {
    
            for _ in 0..width {

                frame_part.push(Mandelbrot::convergence_iterations(max_iteations, &x));
                x.0 += particles.0;
            }
    
            x.0 = real_range_start;
    
            x.1 -= particles.1;
        }

        frame_part
    }

    #[inline(always)]
    fn complex_mul(x: &(f64, f64), y: &(f64, f64)) -> (f64, f64) {

        return (x.0 * y.0 - x.1 * y.1, 2.0 * x.0 * y.1);
    }
}