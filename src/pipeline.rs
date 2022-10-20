use std::{sync::mpsc::Receiver, collections::VecDeque, time::Instant};

use egui::ImageData;
use image::RgbImage;

use crate::utils;

pub struct PipelineOutput {
    pub frame: ImageData,
    pub fps: usize,
}

pub struct Pipeline {
    receiver: Receiver<PipelineOutput>
}

impl Pipeline {
    pub fn new(index: u32, x: u32, y: u32) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            // open camera
            let builder = camera_capture::create(index);
            if let Err(e) = builder {
                eprintln!("could not open camera: {}", e);
                std::process::exit(1);
            }
            
            // init camera
            let image_iterator = builder.unwrap()
                .resolution(x, y).unwrap().start();
            if let Err(e) = image_iterator {
                eprintln!("could retrieve data from camera: {}", e);
                std::process::exit(2);
            }

            let mut images = VecDeque::new();

            // use camera
            let image_iterator = image_iterator.unwrap();
            for image in image_iterator {
                let image = RgbImage::from_raw(image.width(), image.height(), image.into_raw()).unwrap();

                let image_to_display = if let Some((_, last_image)) = images.front() {
                    let image_clone = utils::image_diff(last_image, &image);
                    images.push_front((Instant::now(), image));
                    image_clone
                }
                else {
                    let image_clone = image.clone();
                    images.push_front((Instant::now(), image));
                    image_clone
                };

                images.retain(|(time, _)| time.elapsed().as_secs_f64() < 1.0);

                let pipeline_output = PipelineOutput {
                    frame: utils::rgb_image_to_image_data(&image_to_display, false),
                    fps: images.len()
                };
                
                if sender.send(pipeline_output).is_err() {
                    break;
                }
            }
        });
        
        Pipeline {
            receiver
        }
    }

    pub fn poll(&mut self) -> Option<PipelineOutput> {
        match self.receiver.try_recv() {
            Ok(output) => Some(output),
            Err(_) => None
        }
    }
}