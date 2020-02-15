use raster::{Image, Color};
use palette::{Hsv, rgb::Srgb};
pub struct Frame {
    it_vector: Vec<i32>,
    size: (i32, i32)
}

impl Frame {

    pub fn new(it_vector: Vec<i32>, size: (i32, i32)) -> Frame {
        Frame {
            it_vector: it_vector,
            size: size
        }
    }

    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
        let color_hsv = Hsv::new(h, s, v);
        let color_rgb = Srgb::from(color_hsv);


        return Color::rgb(
            (color_rgb.red * 255.0) as u8,
            (color_rgb.green * 255.0) as u8,
            (color_rgb.blue * 255.0) as u8
        )
    }

    fn determine_color(iterations: &i32, max_iterations: &i32) -> Color {

        return if iterations == max_iterations {
            Color::black()
        } else {
            let iterations = *iterations as f32;
            let max_iterations = *max_iterations as f32;
            let modifier = iterations / max_iterations;

            Frame::hsv_to_rgb(modifier * 180.0, 1.0, modifier)
        }
    }

    pub fn to_image(&self, max_iterations: &i32) -> Image {

        let (width, height) = self.size;
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