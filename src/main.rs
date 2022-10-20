//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// copy escapi.dll to build folder

mod utils;
mod pipeline;

use eframe::egui;
use egui::{ImageData, ColorImage};
use pipeline::Pipeline;

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
    display_texture: Option<egui::TextureHandle>,
    pipeline: Pipeline,
    current_frame: Option<ImageData>,
    current_fps: usize,
}

impl Default for PPPApp {
    fn default() -> Self {
        Self {
            display_texture: None,
            pipeline: Pipeline::new(0, 1280, 720),
            current_frame: Some(ImageData::Color(ColorImage::example())),
            current_fps: 0,
        }
    }
}

impl eframe::App for PPPApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();

            let display_texture = self.display_texture.get_or_insert_with(|| {
                ui.ctx().load_texture(
                    "webcam",
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

            if let Some(output) = self.pipeline.poll() {
                self.current_frame = Some(output.frame);
                self.current_fps = output.fps;
            }

            if let Some(frame) = &self.current_frame {
                display_texture.set(frame.clone(), egui::TextureFilter::Nearest);
                let size = display_texture.size_vec2();
                ui.image(display_texture, size);

                ui.label(format!("FPS: {}", self.current_fps));
            }
        });
    }
}