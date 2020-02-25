pub mod math;
pub mod trans;
pub mod config;

use math::{ComplexRangeF64, ComplexF64, Range};
use trans::FramePart;
use config::FramePartConfig;
use super::utils::worker::Worker;
use std::sync::{Arc, RwLock};

pub enum Fractal {
    Mandelbrot((usize, usize), u16),
    JuliaSet((usize, usize), u16, ComplexF64)
}

impl Fractal {

    pub fn new_thread_safe_generator(gen_type: Fractal) -> Arc<RwLock<dyn FractalGenerator + Send + Sync>> {
        match gen_type {
            Fractal::Mandelbrot(pixel_range, max_iter) => Arc::new(RwLock::new(Mandelbrot::new(pixel_range, max_iter))),
            Fractal::JuliaSet(pixel_range, max_iter, constant) => Arc::new(RwLock::new(JuliaSet::new(pixel_range, max_iter, constant)))
        }
    }

    pub fn generate_frame_on_worker(
        gen_rw_lock: Arc<RwLock<dyn FractalGenerator + Send + Sync>>,
        complex_range: ComplexRangeF64, 
        split_work: usize, 
        worker: &mut Worker<FramePart>,
    ) -> usize {

        let local_rw_lock = gen_rw_lock.clone();

        let generator = local_rw_lock.read().unwrap();

        let particles = generator.between_pixels(complex_range);

        let x = ComplexF64 { 
            re: complex_range.re_range().start(), 
            im: complex_range.im_range().end()
        };
    
        let (width, height) = generator.frame_pixel_size();

        let part_size = height / split_work;
        let leftovers = height % split_work;

        let split_count = split_work + if leftovers != 0 { 1 } else { 0 };

        let max = generator.max_iterations();
        let constant = generator.constant();

        for i in 0..split_work {

            let read_lock = gen_rw_lock.clone();

            worker.push(Box::new(move || -> FramePart {

                let generator = read_lock.read().unwrap();
                let range = Range::new(i * part_size, (i + 1) * part_size);

                let config = FramePartConfig::new(
                    x, range, max, width, particles, constant
                );

                generator.get_frame_part(config)
            }));
        }

        if leftovers != 0 {

            let read_lock = gen_rw_lock.clone();

            worker.push(Box::new(move || -> FramePart {
                let generator = read_lock.read().unwrap();
                let tmp = split_work * part_size;
                let range = Range::new(tmp, tmp + leftovers);

                let config = FramePartConfig::new(
                    x, range, max, width, particles, constant
                );
                
                generator.get_frame_part(config)
            }));
        }

        return split_count;
    }
}

pub trait FractalGenerator {

    fn constant(&self) -> ComplexF64;

    fn frame_pixel_size(&self) -> (usize, usize);

    fn max_iterations(&self) -> u16;

    fn convergence_iterations(&self, max_iter: u16, c: ComplexF64, constant: ComplexF64) -> u16;

    fn between_pixels(&self, complex_range: ComplexRangeF64) -> (f64, f64) {

        if complex_range.re_range().start() >= complex_range.re_range().end() {
            panic!("Invalid real range!");
        }
    
        if complex_range.im_range().start() >= complex_range.im_range().end() {
            panic!("Invalid imaginary range!");
        }
    
        let real_range_len = complex_range.re_range().size();
        let imaginary_range_len = complex_range.im_range().size();

        let pixel_range = self.frame_pixel_size();
    
        (real_range_len as f64 / pixel_range.0 as f64, imaginary_range_len as f64 / pixel_range.1 as f64)
    }

    fn get_frame_part(&self, config: FramePartConfig) -> FramePart {
        
        let start = config.start();
        let mut x = config.start();
        let particles = config.particles();
        let lines = config.lines();

        x.im -= lines.start() as f64 * particles.1;
    
        let mut frame_part: Vec<u16> = Vec::with_capacity(lines.size() * config.width());

        for _ in lines.iterable() {
    
            for _ in 0..config.width() {

                frame_part.push(
                    self.convergence_iterations(config.max_iterations(), x, config.constant())
                );
                x.re += particles.0;
            }
    
            x.re = start.re;
    
            x.im -= particles.1;
        }

        FramePart::new(config.lines(), frame_part)
    }
}

pub struct Mandelbrot {
    pixel_size: (usize, usize),
    max_iter: u16
}

impl Mandelbrot {
    pub fn new(pixel_size: (usize, usize), max_iter: u16) -> Self {
        Mandelbrot {
            pixel_size: pixel_size,
            max_iter: max_iter
        }
    }
}

impl FractalGenerator for Mandelbrot {

    fn constant(&self) -> ComplexF64 {
        ComplexF64 {
            re: 0.0,
            im: 0.0
        }
    }

    fn frame_pixel_size(&self) -> (usize, usize) {
        self.pixel_size
    }

    fn max_iterations(&self) -> u16 {
        self.max_iter
    }

    fn convergence_iterations(&self, max_iter: u16, c: ComplexF64, constant: ComplexF64) -> u16 {
        let mut i = 0;
        let mut result = constant;

        while result.re * result.re + result.im * result.im < 4.0 && i < max_iter {

            result = ComplexF64::mul(result, result);
            result = ComplexF64 {
                re: result.re + c.re, im: result.im + c.im
            };

            i += 1;
        }

        return i;
    }
}

pub struct JuliaSet {
    pixel_size: (usize, usize),
    max_iter: u16,
    constant: ComplexF64
}

impl JuliaSet {
    pub fn new(pixel_size: (usize, usize), max_iter: u16, constant: ComplexF64) -> Self {
        JuliaSet {
            pixel_size: pixel_size,
            max_iter: max_iter,
            constant: constant
        }
    }
}

impl FractalGenerator for JuliaSet {

    fn constant(&self) -> ComplexF64 {
        self.constant
    }

    fn frame_pixel_size(&self) -> (usize, usize) {
        self.pixel_size
    }

    fn max_iterations(&self) -> u16 {
        self.max_iter
    }

    fn convergence_iterations(&self, max_iter: u16, c: ComplexF64, constant: ComplexF64) -> u16 {
        let mut i = 0;
        let mut result = c;

        while result.re * result.re + result.im * result.im < 4.0 && i < max_iter {

            result = ComplexF64::mul(result, result);
            result = ComplexF64 {
                re: result.re + constant.re, im: result.im + constant.im
            };

            i += 1;
        }

        return i;
    }
}