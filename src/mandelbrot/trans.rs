use raster::{Image, Color};

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

    fn determine_color(iterations: &i32, max_iterations: &i32) -> Color {

        return if iterations >= max_iterations {
            Color::black()
        } else {
            Color::white()
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