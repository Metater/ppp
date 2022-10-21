//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// copy escapi.dll to build folder

mod utils;
mod pipeline;

use std::sync::{atomic::{AtomicU8, Ordering}, Arc};

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
    current_processing_time: f64,

    setting_threshold: Arc<AtomicU8>,
    setting_mut_threshold: u8,

    setting_block_size: Arc<AtomicU8>,
    setting_mut_block_size: u8,
}

impl Default for PPPApp {
    fn default() -> Self {
        let setting_threshold = Arc::new(AtomicU8::new(0));

        let setting_block_size = Arc::new(AtomicU8::new(1));

        Self {
            display_texture: None,
            pipeline: Pipeline::new(0, 1280, 720, setting_threshold.clone(), setting_block_size.clone()),
            current_frame: Some(ImageData::Color(ColorImage::example())),
            current_fps: 0,
            current_processing_time: 0.0,

            setting_threshold,
            setting_mut_threshold: 50,

            setting_block_size,
            setting_mut_block_size: 1,
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
                self.current_processing_time = output.processing_time;
            }

            if let Some(frame) = &self.current_frame {
                display_texture.set(frame.clone(), egui::TextureFilter::Nearest);
                let size = display_texture.size_vec2();
                ui.image(display_texture, size);

                ui.label(format!("FPS: {}", self.current_fps));
                ui.label(format!("Processing Time: {}", self.current_processing_time));

                ui.add(egui::Slider::new(&mut self.setting_mut_threshold, 0..=255).text("Threshold"));
                self.setting_threshold.store(self.setting_mut_threshold, Ordering::Relaxed);

                ui.add(egui::Slider::new(&mut self.setting_mut_block_size,1..=128).text("Block Size"));
                self.setting_block_size.store(self.setting_mut_block_size, Ordering::Relaxed);
            }
        });
    }
}