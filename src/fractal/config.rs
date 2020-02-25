use serde::{Serialize, Deserialize};
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use super::math::{Range, ComplexRangeF64, ComplexF64};


#[derive(Serialize, Deserialize)]
pub struct ImageConfig {
    re_range: (f64, f64),
    im_range: (f64, f64),
    pixel_range: (usize, usize),
    max_iterations: u16,
    threads: usize,
    thread_split: usize
}

impl ImageConfig {
    pub fn default() -> Self {
        ImageConfig {
            re_range: (-2.0, 0.5),
            im_range: (-1.0, 1.0),
            pixel_range: (1250, 1000),
            max_iterations: 1000,
            threads: 16,
            thread_split: 1
        }
    }

    pub fn read_form_file_or_default(path: &str) -> ImageConfig {

        let config_path = Path::new(path);

        if !config_path.exists() {

            let json_config = serde_json::to_string_pretty(&ImageConfig::default()).unwrap();

            File::create(config_path).and_then(|mut file| file.write_all(&json_config.as_bytes())).unwrap();
        }

        let mut json_config = String::new();
        File::open(config_path).and_then(|mut file| file.read_to_string(&mut json_config)).unwrap();
        
        serde_json::from_str(&json_config).unwrap()
    }

    pub fn re_range(&self) -> Range<f64> {
        Range::new_from_tuple(self.re_range)
    }

    pub fn im_range(&self) -> Range<f64> {
        Range::new_from_tuple(self.im_range)
    }

    pub fn complex_range(&self) -> ComplexRangeF64 {
        ComplexRangeF64::new(
            self.re_range(),
            self.im_range()
        )
    }

    pub fn pixel_range(&self) -> (usize, usize) {
        self.pixel_range
    }

    pub fn max_iterations(&self) -> u16 {
        self.max_iterations
    }

    pub fn threads(&self) -> usize {
        self.threads
    }

    pub fn thread_split(&self) -> usize {
        self.thread_split
    }
}

pub struct FramePartConfig {
    start: ComplexF64, 
    lines: Range<usize>,
    max_iter: u16,
    width: usize,
    particles: (f64, f64),
    constant: ComplexF64
}

impl FramePartConfig {

    pub fn new(
        start: ComplexF64, 
        lines: Range<usize>,
        max_iter: u16,
        width: usize, 
        particles: (f64, f64),
        constant: ComplexF64
    ) -> Self {

        FramePartConfig {
            start: start, 
            lines: lines,
            max_iter: max_iter,
            width: width,
            particles: particles,
            constant: constant
        }
    }

    pub fn start(&self) -> ComplexF64 {
        self.start
    }

    pub fn lines(&self) -> Range<usize> {
        self.lines
    }

    pub fn max_iterations(&self) -> u16 {
        self.max_iter
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn particles(&self) -> (f64, f64) {
        self.particles
    }

    pub fn constant(&self) -> ComplexF64 {
        self.constant
    }
}