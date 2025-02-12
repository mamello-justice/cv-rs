//! Using Prewitt operator for edge detection

use image::{GrayImage, Luma};

use convolution::{Kernel, PaddingType};

use crate::core::EdgeDetection;

pub struct PrewittOperator {}

impl PrewittOperator {
    pub fn new() -> PrewittOperator {
        PrewittOperator {}
    }
}

impl EdgeDetection for PrewittOperator {
    fn detect_edge(&self, image: &GrayImage) -> GrayImage {
        let mut gx_kernel = Kernel::new(3, 3, vec![1, 0, -1, 1, 0, -1, 1, 0, -1]);
        gx_kernel.padding = PaddingType::Zero;

        let mut gy_kernel = Kernel::new(3, 3, vec![1, 1, 1, 0, 0, 0, -1, -1, -1]);
        gy_kernel.padding = PaddingType::Zero;

        let gx = gx_kernel.convolve(image);
        let gy = gy_kernel.convolve(image);

        let mut g = GrayImage::new(image.width(), image.height());

        for y in 0..image.height() {
            for x in 0..image.width() {
                let gx_val = gx.get_pixel(x, y).0[0] as f64;
                let gy_val = gy.get_pixel(x, y).0[0] as f64;
                let pixel = (gx_val.powf(2.0) + gy_val.powf(2.0)).sqrt();
                g.put_pixel(x, y, Luma([pixel as u8]));
            }
        }

        g
    }
}
