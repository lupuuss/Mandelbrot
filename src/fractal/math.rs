use std::ops::Sub;
use std::ops::Mul;
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct ComplexF64 {
    pub re: f64,
    pub im: f64
}

impl ComplexF64 {

    #[inline]
    pub fn norm_2(&self) -> f64 {
        return self.re * self.re + self.im * self.im
    }
}

impl Mul for ComplexF64  {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {

        return ComplexF64 {
            re: self.re * rhs.re - self.im * rhs.im,
            im: 2.0 * self.re * rhs.im
        }
    }
    
}

impl Add for ComplexF64 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        return ComplexF64 {
            re: self.re + rhs.re,
            im: self.im + rhs.im
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