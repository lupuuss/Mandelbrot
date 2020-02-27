use sdl2::surface::Surface;
use sdl2::image::SaveSurface;
use sdl2::video::WindowSurfaceRef;
use sdl2::pixels::Color;

use std::path::Path;
use std::fs::File;

use palette::{Hsv, rgb::Srgb};
use super::math::Range;


#[allow(dead_code)]
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let color_hsv = Hsv::new(h, s, v);
    let color_rgb = Srgb::from(color_hsv);


    return Color::RGB(
        (color_rgb.red * 255.0) as u8,
        (color_rgb.green * 255.0) as u8,
        (color_rgb.blue * 255.0) as u8
    )
}

fn determine_color(iterations: &u16, max_iterations: &u16) -> Color {

    return if iterations == max_iterations {
        Color::RGB(0, 0, 0)

    } else {
        let iterations: u16 = iterations << 2;
        let modifier = iterations as f32 / 1000.0;

        hsv_to_rgb(
            modifier * 360.0, 1.0, 1.0
        )
    }
}

pub struct FramePart {
    lines: Range<usize>,
    it_vector: Vec<u16>
}

impl FramePart {
    pub fn new(lines: Range<usize>, it_vector: Vec<u16>) -> FramePart {
        FramePart {
            lines: lines,
            it_vector: it_vector
        }
    }

    pub fn range(&self) -> Range<usize> {
        self.lines
    }

    pub fn vector(&self) -> &Vec<u16> {
        &self.it_vector
    }
}

pub trait GeneralizedSurface {
    fn manipulate<F:  FnOnce(&mut [u8]) -> ()>(&mut self, f: F);
    fn get_size(&self) -> (u32, u32);
}

impl<'a> GeneralizedSurface for WindowSurfaceRef<'a> {

    fn manipulate<F:  FnOnce(&mut [u8]) -> ()>(&mut self, f: F) {
        self.with_lock_mut(f);
    }

    fn get_size(&self) -> (u32, u32) {
        self.size()
    }
}

impl<'a> GeneralizedSurface for Surface<'a> {

    fn manipulate<F:  FnOnce(&mut [u8]) -> ()>(&mut self, f: F) {
        self.with_lock_mut(f);
    }

    fn get_size(&self) -> (u32, u32) {
        self.size()
    }
}

pub struct SurfaceWriter<T> {
    surface: T
}

impl<'a> SurfaceWriter<Surface<'a>> {
    
    pub fn new_blank(width: u32, hegiht: u32) -> Self {
        SurfaceWriter {
            surface: Surface::new(width, hegiht, sdl2::pixels::PixelFormatEnum::RGB888).unwrap()
        }
    }
}

impl<T: GeneralizedSurface> SurfaceWriter<T> {

    pub fn new(surface: T) -> Self {
        SurfaceWriter {
            surface: surface
        }
    }

    pub fn write_part(&mut self, frame_part: FramePart, max_iter: u16) {

        let width = (self.surface.get_size().0) as usize;

        let bytes = 4;
        let r_byte = 2;
        let g_byte = 1;
        let b_byte = 0;

        self.surface.manipulate(|pixels| -> () {
 
            let absolute = frame_part.lines.start() * width * bytes;

            for (i, iter) in frame_part.vector().iter().enumerate() {
            
                let color = determine_color(iter, &max_iter);

                pixels[absolute + i * bytes + r_byte] = color.r;
                pixels[absolute + i * bytes + g_byte] = color.g;
                pixels[absolute + i * bytes + b_byte] = color.b;
            }
        });
    }
}

impl<T: SaveSurface> SurfaceWriter<T> {

    pub fn save_to_image(self, path: &str) -> Result<(), String> {

        let path = Path::new(path);

        if !path.exists() {
            File::create(path).unwrap();
        }

        return self.surface.save(path);
    }
}

impl<'a> SurfaceWriter<WindowSurfaceRef<'a>> {
    pub fn update_window(&mut self) -> Result<(), String> {
        return self.surface.update_window();
    }
}