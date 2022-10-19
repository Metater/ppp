use egui::{ImageData, Color32, ColorImage};
use image::{ImageBuffer, Rgb};

pub fn to_image_data(image: &ImageBuffer<Rgb<u8>, Vec<u8>>, is_grayscale: bool) -> ImageData {
    let pixels = image.pixels().map(|p| {
        if is_grayscale {
            Color32::from_gray((0.299*p[0] as f32 + 0.587*p[1] as f32 + 0.114*p[2] as f32) as u8)
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