use super::math::ComplexF64;
use super::FractalGenerator;


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