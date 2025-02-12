//! Basic Roberts cross edge detection
//!
//! Finds the edges using the Roberts cross edge detector for grayscale images and basic kernel convolution.
//!
//! L. Roberts Machine Perception of 3-D Solids, Optical and Electro-optical Information Processing, MIT Press 1965

use image::{GrayImage, Luma};

use convolution::{Kernel, Padding};

use crate::core::EdgeDetection;

pub struct RobertsCross {}

impl RobertsCross {
    pub fn new() -> RobertsCross {
        RobertsCross {}
    }
}

impl EdgeDetection for RobertsCross {
    fn detect_edge(&self, image: &GrayImage) -> GrayImage {
        let mut gx_kernel = Kernel::new(2, 2, vec![1, 0, 0, -1]);
        gx_kernel.padding = Padding::Zero;

        let mut gy_kernel = Kernel::new(2, 2, vec![0, 1, -1, 0]);
        gy_kernel.padding = Padding::Zero;

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
