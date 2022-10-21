use egui::{ImageData, Color32, ColorImage};
use image::{RgbImage, Rgb, GrayImage, Luma};

pub fn _rgb_image_to_image_data(image: &RgbImage, is_grayscale: bool) -> ImageData {
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

pub fn _gray_image_to_image_data(image: &GrayImage) -> ImageData {
    let pixels = image.pixels().map(|p| {
        Color32::from_gray(p.0[0])
    }).collect();

    let dimensions = image.dimensions();
    let size = [dimensions.0 as usize, dimensions.1 as usize];

    ImageData::Color(ColorImage {
        size,
        pixels
    })
}

pub fn _rgb_image_to_gray_image(image_a: &RgbImage) -> GrayImage {
    let mut new_image = GrayImage::new(image_a.width(), image_a.height());
    new_image.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let pixel_a = image_a.get_pixel(x, y);

        *pixel = _rgb_pixel_to_luma_pixel(pixel_a);
    });
    new_image
}

pub fn _rgb_pixel_to_luma_pixel(pixel: &Rgb<u8>) -> Luma<u8> {
    Luma([((0.299 * pixel[0] as f32) + (0.587 * pixel[1] as f32) + (0.114 * pixel[2] as f32)) as u8])
}

pub fn _f32_image_to_gray_image(image: &Vec<f32>, width: u32, height: u32) -> GrayImage {
    let mut new_image = GrayImage::new(width, height);
    new_image.pixels_mut().enumerate().for_each(|(i, pixel)| {
        *pixel = Luma([image[i] as u8]);
    });
    new_image
}

pub fn _rgb_image_diff(image_a: &RgbImage, image_b: &RgbImage) -> RgbImage {
    let mut new_image = RgbImage::new(image_a.width(), image_a.height());
    new_image.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let pixel_a = image_a.get_pixel(x, y);
        let pixel_b = image_b.get_pixel(x, y);

        let new_pixel = [
            (pixel_a[0] as i32 - pixel_b[0] as i32).abs() as u8,
            (pixel_a[1] as i32 - pixel_b[1] as i32).abs() as u8,
            (pixel_a[2] as i32 - pixel_b[2] as i32).abs() as u8
        ];

        *pixel = Rgb(new_pixel);
    });
    new_image
}

pub fn _gray_image_diff(image_a: &GrayImage, image_b: &GrayImage) -> GrayImage {
    let mut new_image = GrayImage::new(image_a.width(), image_a.height());
    new_image.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let pixel_a = image_a.get_pixel(x, y);
        let pixel_b = image_b.get_pixel(x, y);

        let new_pixel = [
            (pixel_a[0] as i32 - pixel_b[0] as i32).abs() as u8
        ];

        *pixel = Luma(new_pixel);
    });
    new_image
}