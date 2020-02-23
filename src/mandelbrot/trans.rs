use raster::{Image, Color};
use palette::{Hsv, rgb::Srgb};

#[allow(dead_code)]
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let color_hsv = Hsv::new(h, s, v);
    let color_rgb = Srgb::from(color_hsv);


    return Color::rgb(
        (color_rgb.red * 255.0) as u8,
        (color_rgb.green * 255.0) as u8,
        (color_rgb.blue * 255.0) as u8
    )
}

fn determine_color(iterations: &u16, max_iterations: &u16) -> Color {

    return if iterations == max_iterations {
        Color::black()
    } else {
        let iterations: u16 = iterations << 2;
        let modifier = iterations as f32 / *max_iterations as f32;

        hsv_to_rgb(
            modifier * 360.0, 1.0, 1.0
        )
    }
}

pub struct FramePart {
    range: (usize, usize),
    it_vector: Vec<u16>
}

impl FramePart {
    pub fn new(range: (usize, usize), it_vector: Vec<u16>) -> FramePart {
        FramePart {
            range: range,
            it_vector: it_vector
        }
    }

    pub fn range(&self) -> (usize, usize) {
        self.range
    }
}

pub struct ImageWriter {
    size: (usize, usize),
    image: Image
}

impl ImageWriter {

    pub fn new(size: (usize, usize)) -> ImageWriter {
        ImageWriter {
            size: size,
            image: Image::blank(size.0 as i32, size.1 as i32)
        }
    }

    pub fn write_part(&mut self, part: FramePart, max_iterations: u16)  {
        
        let width = self.size.0 as i32;

        let absolute = part.range.0 as i32 * width;

        for (i, iterations) in part.it_vector.iter().enumerate() {
            
            let pixel_number = absolute + i as i32;

            self.image.set_pixel(
                pixel_number % width,
                pixel_number / width,
                self::determine_color(iterations, &max_iterations)
            ).unwrap();
        }
    }

    pub fn to_image(self) -> Image {
        self.image
    }
}