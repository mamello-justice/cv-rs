use image::{GrayImage, ImageReader};

use edge_detection::core::EdgeDetection;
use edge_detection::sobel::SobelOperator;

#[test]
fn preserve_size() {
    let detector = SobelOperator::new();
    let original = GrayImage::new(15, 10);
    let result = detector.detect_edge(&original);
    assert_eq!(result.width(), original.width());
    assert_eq!(result.height(), original.height());
}

#[test]
fn finds_edge() {
    let detector = SobelOperator::new();
    let original = ImageReader::open("./data/test.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let grayscale: GrayImage = original.grayscale().try_into().unwrap();
    let result = detector.detect_edge(&grayscale);
    result.save("./data/test_sobel.png").unwrap();
}
