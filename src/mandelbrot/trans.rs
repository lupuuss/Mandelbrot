use raster::{Image, Color};
use palette::{Hsv, rgb::Srgb};
pub struct Frame {
    it_vector: Vec<u16>,
    size: (usize, usize)
}

impl Frame {

    pub fn new(it_vector: Vec<u16>, size: (usize, usize)) -> Frame {
        Frame {
            it_vector: it_vector,
            size: size
        }
    }

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
            let iterations = iterations * 500;
            Color::rgb(
                0, (iterations >> 8) as u8, ((iterations << 8) >> 8) as u8
            )
        }
    }

    pub fn to_image(&self, max_iterations: &u16) -> Image {

        let width = self.size.0 as i32;
        let height = self.size.1 as i32;

        let mut image = Image::blank(width, height);

        for (index, iterations) in self.it_vector.iter().enumerate() {  
            
            image.set_pixel(
                index as i32 % width,
                index as i32 / width,
                Frame::determine_color(iterations, &max_iterations)
            ).unwrap();
        }

        image
    }
}