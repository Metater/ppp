//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// copy escapi.dll to build folder

mod webcam;
mod utils;

use std::{time::Instant, sync::mpsc::Receiver};

use camera_capture::ImageIterator;
use eframe::egui;
use egui::{ImageData, ColorImage, Color32, Context};
use image::{ImageBuffer, Rgb, RgbImage};
extern crate camera_capture;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.vsync = true;
    eframe::run_native(
        "PPP",
        options,
        Box::new(|_cc| Box::new(PPPApp::default())),
    );
}

struct PPPApp {
    webcam_texture: Option<egui::TextureHandle>,
    webcam: Receiver<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    current_frame: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
}

impl Default for PPPApp {
    fn default() -> Self {
        Self {
            webcam_texture: None,
            webcam: webcam::start(),
            current_frame: Some(ImageBuffer::new(640, 480)),
        }
    }
}

impl eframe::App for PPPApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();

            // INIT
            let webcam_texture = self.webcam_texture.get_or_insert_with(|| {
                ui.ctx().load_texture(
                    "webcam",
                    //egui::ColorImage::example(),
                    ImageData::Color(ColorImage::example()),
                    egui::TextureFilter::Nearest
                )
            });

            ui.heading("Hello, World!");
            /*
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            */

            let now = Instant::now();

            if let Ok(frame) = self.webcam.try_recv() {
                /*
                let mut diff = RgbImage::new(640, 480);
                if let Some(old_frame) = &self.current_frame {
                    for y in 0..480 {
                        for x in 0..640 {
                            let new_pixel = frame.get_pixel(x, y);
                            let old_pixel = old_frame.get_pixel(x, y);
                            let pixel_diff = Rgb([
                                (new_pixel[0] as i32 - old_pixel[0] as i32).abs() as u8,
                                (new_pixel[1] as i32 - old_pixel[1] as i32).abs() as u8,
                                (new_pixel[2] as i32 - old_pixel[2] as i32).abs() as u8,
                            ]);
                            diff.put_pixel(x, y, pixel_diff)
                        }
                    }
                }
                */

                let image_data = utils::to_image_data(&frame, false);

                webcam_texture.set(image_data, egui::TextureFilter::Nearest);
                let size = webcam_texture.size_vec2();
                ui.image(webcam_texture, size);

                self.current_frame = Some(frame);
            }
            else {
                if let Some(old_frame) = &self.current_frame {
                    let image_data = utils::to_image_data(&old_frame, false);

                    webcam_texture.set(image_data, egui::TextureFilter::Nearest);
                    let size = webcam_texture.size_vec2();
                    ui.image(webcam_texture, size);
                }
            }

            println!("{}", now.elapsed().as_millis());
        });
    }
}