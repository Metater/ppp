//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{time::Instant, sync::mpsc::Receiver};

use camera_capture::ImageIterator;
use eframe::egui;
use egui::{ImageData, ColorImage, Color32, Context};
use image::{ImageBuffer, Rgb};
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
    name: String,
    age: u32,
    texture: Option<egui::TextureHandle>,
    webcam: std::sync::mpsc::Receiver<ImageData>,
    current_frame: Option<ImageData>,
}

impl Default for PPPApp {
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        let _ = std::thread::spawn(move || {
            let res1 = camera_capture::create(0);
            if let Err(e) = res1 {
                eprintln!("could not open camera: {}", e);
                std::process::exit(1);
            }
            let res2 = res1.unwrap().resolution(640, 480).unwrap().fps(30.0).unwrap().start();
            if let Err(e) = res2 {
                eprintln!("could retrieve data from camera: {}", e);
                std::process::exit(2);
            }
            let cam = res2.unwrap();
            for frame in cam {
                let mut pixels = Vec::new();
                for pixel in frame.pixels() {
                    let mut avg = pixel[0] as i16 + pixel[1] as i16 + pixel[2] as i16;
                    avg /= 3;
                    pixels.push(Color32::from_rgb(avg as u8, avg as u8, avg as u8));
                }
                let dimensions = frame.dimensions();
                let size = [dimensions.0 as usize, dimensions.1 as usize];

                let image = ImageData::Color(ColorImage {
                    size,
                    pixels
                });
                if sender.send(image).is_err() {
                    break;
                }
            }
        });
        
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            texture: None,
            webcam: receiver,
            current_frame: None,
        }
    }
}

impl eframe::App for PPPApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();
            let now = Instant::now();
            ui.heading("Hello, World!");
            println!("test??");
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
            let texture = self.texture.get_or_insert_with(|| {
                ui.ctx().load_texture(
                    "webcam",
                    //egui::ColorImage::example(),
                    ImageData::Color(ColorImage::example()),
                    egui::TextureFilter::Nearest
                )
            });

            //ui.heading(self.camera.is_stream_open().to_string());
            //let frame = image::io::Reader::open("D:/Webcam/frame.png").unwrap().decode().unwrap();
            if let Ok(frame) = self.webcam.try_recv() {
                self.current_frame = Some(frame.clone());
                texture.set(frame, egui::TextureFilter::Nearest);
                let size = texture.size_vec2();
                ui.image(texture, size);
            }
            else {
                if let Some(frame) = &self.current_frame {
                    texture.set(frame.clone(), egui::TextureFilter::Nearest);
                    let size = texture.size_vec2();
                    ui.image(texture, size);
                }
            }

            println!("{}", now.elapsed().as_millis());
        });
    }
}