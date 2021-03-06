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

impl<T: Add<Output = T> + Copy> Range<T> {

    pub fn shift(&mut self, mv: T) {
        self.start = self.start + mv;
        self.end = self.end + mv;
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Copy> Range<T> {

    pub fn shrink(&mut self, shrink: T) {
        self.start = self.start + shrink;
        self.end = self.end - shrink;
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

    pub fn move_range(&mut self, mv: (f64, f64)) {
        self.re_range.shift(mv.0);
        self.im_range.shift(mv.1);
    }

    fn safe_shrink(range: &Range<f64>, shrink: f64) -> f64 {

        if shrink > range.size() / 2.0 {

            return range.size()  * 0.48;

        } else {

            return shrink;
        }
    }

    pub fn shrink_range(&mut self, shrink: (f64, f64)) {


        self.re_range.shrink(
            ComplexRangeF64::safe_shrink(&self.re_range, shrink.0)
        );
        self.im_range.shrink(
            ComplexRangeF64::safe_shrink(&self.im_range, shrink.1)
        );
    }
}