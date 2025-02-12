use image::{GrayImage, Luma};

pub enum PaddingType {
    Zero,
    None,
}

pub struct Padding {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
    pub x: u32,
    pub y: u32,
}

pub struct Kernel {
    pub width: u32,
    pub height: u32,
    pub data: Vec<i32>,
    pub padding: PaddingType,
}

impl Kernel {
    pub fn new(width: u32, height: u32, data: Vec<i32>) -> Self {
        Kernel {
            width,
            height,
            data,
            padding: PaddingType::None,
        }
    }

    pub fn get_weight(&self, x: u32, y: u32) -> i32 {
        self.data[usize::try_from(x + y * self.width).unwrap()]
    }

    pub fn get_padding(&self) -> Padding {
        let (left, right) = if self.width % 2 == 0 {
            (0, self.width / 2)
        } else {
            let pad = (self.width + 1) / 4;
            (pad, pad)
        };

        let (top, bottom) = if self.height % 2 == 0 {
            (0, self.height / 2)
        } else {
            let pad = (self.height + 1) / 4;
            (pad, pad)
        };

        Padding {
            left,
            right,
            top,
            bottom,
            x: left + right,
            y: top + bottom,
        }
    }

    pub fn convolve(&self, input: &GrayImage) -> GrayImage {
        let image_width = input.width();
        let image_height = input.height();

        let padding = self.get_padding();

        let mut padded = GrayImage::new(image_width + padding.x, image_height + padding.y);
        for y in 0..image_height {
            for x in 0..image_width {
                padded.put_pixel(
                    x + padding.left,
                    y + padding.top,
                    input.get_pixel(x, y).to_owned(),
                );
            }
        }

        let mut result = GrayImage::new(image_width, image_height);
        for y in 0..image_height {
            for x in 0..image_width {
                let mut g = 0;
                for kx in 0..self.height {
                    for ky in 0..self.width {
                        g += self.get_weight(kx, ky) * padded.get_pixel(x + kx, y + ky).0[0] as i32;
                    }
                }
                let pixel = if g < u8::MIN as i32 {
                    u8::MIN
                } else if g > u8::MAX as i32 {
                    u8::MAX
                } else {
                    g as u8
                };
                result.put_pixel(x, y, Luma([pixel.try_into().unwrap()]));
            }
        }

        result
    }
}
