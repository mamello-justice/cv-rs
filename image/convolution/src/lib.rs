use image::{GrayImage, Luma, Pixel};

pub enum Padding {
    Zero,
    None,
}

pub struct Kernel {
    pub width: u32,
    pub height: u32,
    pub data: Vec<i32>,
    pub padding: Padding,
}

impl Kernel {
    pub fn new(width: u32, height: u32, data: Vec<i32>) -> Self {
        Kernel {
            width,
            height,
            data,
            padding: Padding::None,
        }
    }

    pub fn get_weight(&self, x: u32, y: u32) -> i32 {
        self.data[usize::try_from(x + y * self.width).unwrap()]
    }

    pub fn convolve(&self, input: &GrayImage) -> GrayImage {
        // TODO: This only considers 2x2 kernels

        let image_width = input.width();
        let image_height = input.height();

        let mut padded = GrayImage::new(image_width + 1, image_height + 1);
        for y in 0..image_height {
            for x in 0..image_width {
                padded.put_pixel(x, y, input.get_pixel(x, y).to_owned());
            }
        }

        let mut result = GrayImage::new(image_width, image_height);
        for y in 0..image_height {
            for x in 0..image_width {
                let mut g = 0;
                for kx in 0..self.height {
                    for ky in 0..self.width {
                        g += self.get_weight(kx, ky)
                            * padded.get_pixel(x + kx, y + ky).channels()[0] as i32;
                    }
                }
                let pixel = if g < 0 { 0 } else { g };
                result.put_pixel(x, y, Luma([pixel.try_into().unwrap()]));
            }
        }

        result
    }
}
