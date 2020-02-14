pub mod trans;

use std::vec::Vec;
use trans::Frame;
use std::thread::spawn;
use std::sync::{Arc, Barrier};

pub struct Mandelbrot {
    max_iteations: i32,
    pixel_range: (i32, i32)
}

impl Mandelbrot {

    pub fn get_iterations_frame(&self, complex_range: ((f64, f64), (f64, f64)), split_work: i32) -> Frame {

        let particles = self.between_pixels(complex_range);
        let mut frame: Vec<i32> = vec![];
    
        let (re_range, im_range) = complex_range;

        let x = (re_range.0, im_range.1);
    
        let (width, height) = self.pixel_range;

        let part_size = height / split_work;
        let leftovers = height % split_work;

        let threads_count = split_work + if leftovers != 0 { 1 } else { 0 };

        let mut barrier = Arc::new(Barrier::new(threads_count as usize));

        let mut handles = Vec::with_capacity(threads_count as usize);

        let max = self.max_iteations;

        for i in 0..split_work as i32 {
            let c = barrier.clone();
            handles.push(spawn(move || -> Vec<i32> {
                
                let result = Mandelbrot::get_frame_part(x, &max, (i * part_size, ((i + 1) * part_size)), width, particles);
                c.wait();
                return result;
            }));
        }

        if leftovers != 0 {
            let c = barrier.clone();

            handles.push(spawn(move || -> Vec<i32> {
                let tmp = split_work * part_size;
                let result = Mandelbrot::get_frame_part(x, &max, (tmp, tmp + leftovers), width, particles);
                c.wait();
                return result;
            }))
        }

        for handle in handles {
            frame.append(&mut handle.join().unwrap());
        }
        

        Frame::new(frame, self.pixel_range)
    
    }

    pub fn new(max_iteations: i32, pixel_range: (i32, i32)) -> Mandelbrot {
        Mandelbrot {
            max_iteations: max_iteations,
            pixel_range: pixel_range
        }
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

    fn convergence_iterations(max_iteations: &i32, n: &(f64, f64)) -> i32 {
        
        let mut i = 0;
        let mut result = (0.0, 0.0);

        while result.0 * result.0 + result.1 * result.1 < 4.0 && i < *max_iteations {

            result = Mandelbrot::complex_mul(&result, &result);
            result = (result.0 + n.0, result.1 + n.1);
            i += 1;
        }

        return i;
    }

    fn get_frame_part(
        start: (f64, f64), 
        max_iteations: &i32, 
        lines: (i32, i32), 
        width: i32, 
        particles: (f64, f64)
    ) -> Vec<i32> {

        let mut x = start;
        x.1 -= lines.0 as f64 * particles.1;

        let real_range_start = start.0;
    
        let mut frame_part: Vec<i32> = vec![];

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