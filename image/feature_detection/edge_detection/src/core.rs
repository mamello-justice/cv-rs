use image::GrayImage;

pub trait EdgeDetection {
    fn detect_edge(&self, image: &GrayImage) -> GrayImage;
}
