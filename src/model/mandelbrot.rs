pub struct Mandelbrot {
    max_iteations: i32,
    complex_range: ((f64, f64),(f64, f64)),
    pixel_range: (i32, i32)
}

impl Mandelbrot {

    pub fn new(max_iteations: i32, complex_range: ((f64, f64),(f64, f64)), pixel_range: (i32, i32)) -> Mandelbrot {
        Mandelbrot {
            max_iteations: max_iteations,
            complex_range: complex_range,
            pixel_range: pixel_range
        }
    }

    pub fn between_pixels(&self) -> (f64, f64) {

        let (real_range, imaginary_range) = self.complex_range;

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

    pub fn convergence_iterations(&self, n: &(f64, f64)) -> i32 {
        let mut i = 0;
        let mut result = (0.0, 0.0);

        while i < self.max_iteations && result.0 * result.0 + result.1 * result.1 < 4.0  {

            result = Mandelbrot::complex_mul(&result, &result);
            result = (result.0 + n.0, result.1 + n.1);
            i += 1;
        }

        return i;
    }

    pub fn get_real_range(&self) -> (f64, f64) {
        return self.complex_range.0;
    }

    pub fn get_imaginary_range(&self) -> (f64, f64) {
        return self.complex_range.1;
    }

    pub fn get_pixel_range(&self) -> (i32, i32) {
        return self.pixel_range;
    }

    pub fn get_max_iterations(&self) -> i32 {
        return self.max_iteations;
    }

    pub fn get_start_point(&self) -> (f64, f64) {
        return (self.get_real_range().0, self.get_imaginary_range().1);
    }


    fn complex_mul(x: &(f64, f64), y: &(f64, f64)) -> (f64, f64) {

        return (x.0 * y.0 - x.1 * y.1, 2.0 * x.0 * y.1);
    }
}