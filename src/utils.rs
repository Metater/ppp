use egui::{ImageData, Color32, ColorImage};
use image::{RgbImage, Rgb};

pub fn rgb_image_to_image_data(image: &RgbImage, is_grayscale: bool) -> ImageData {
    let pixels = image.pixels().map(|p| {
        if is_grayscale {
            Color32::from_gray((0.299 * p[0] as f32 + 0.587 * p[1] as f32 + 0.114 * p[2] as f32) as u8)
        }
        else {
            Color32::from_rgb(p[0], p[1], p[2])
        }
    }).collect();

    let dimensions = image.dimensions();
    let size = [dimensions.0 as usize, dimensions.1 as usize];

    ImageData::Color(ColorImage {
        size,
        pixels
    })
}

pub fn image_diff(image_a: &RgbImage, image_b: &RgbImage) -> RgbImage {
    let mut diff = RgbImage::new(image_a.width(), image_a.height());
    for (x, y, pixel) in diff.enumerate_pixels_mut() {
        let pixel_a = image_a.get_pixel(x, y);
        let pixel_b = image_b.get_pixel(x, y);

        let diff = [
            (pixel_a[0] as i32 - pixel_b[0] as i32).abs() as u8,
            (pixel_a[1] as i32 - pixel_b[1] as i32).abs() as u8,
            (pixel_a[2] as i32 - pixel_b[2] as i32).abs() as u8
        ];

        *pixel = Rgb(diff);
    }
    diff
}