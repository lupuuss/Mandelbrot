use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct ComplexF64 {
    pub re: f64,
    pub im: f64
}

impl ComplexF64 {

    #[inline(always)]
    pub fn mul(a: ComplexF64, b: ComplexF64) -> ComplexF64 {

        return ComplexF64 {
            re: a.re * b.re - a.im * b.im,
            im: 2.0 * a.re * b.im
        }
    }
}

#[derive(Copy, Clone)]
pub struct Range<T: Copy> {
    start: T,
    end: T
}

impl<T: Copy> Range<T> {

    pub fn new_from_tuple(range: (T, T)) -> Self {
        Range::new(
            range.0,
            range.1
        )
    }

    pub fn new(start: T, end: T) -> Self {

        Range {
            start: start,
            end: end
        }
    }

    pub fn start(&self) -> T {
        self.start
    }

    pub fn end(&self) -> T {
        self.end
    }
}

impl<T: Sub<Output = T> + Copy> Range<T> {

    pub fn size(&self) -> T {
        self.end - self.start
    }
}

impl Range<usize> {
    pub fn iterable(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}

#[derive(Copy, Clone)]
pub struct ComplexRangeF64 {
    re_range: Range<f64>,
    im_range: Range<f64>
}

impl ComplexRangeF64 {

    pub fn new(re_range: Range<f64>, im_range: Range<f64>) -> Self {
        ComplexRangeF64 {
            re_range: re_range,
            im_range: im_range
        }
    }

    pub fn re_range(&self) -> Range<f64> {
        self.re_range
    }

    pub fn im_range(&self) -> Range<f64> {
        self.im_range
    }
}