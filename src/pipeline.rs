use std::{sync::{mpsc::Receiver, atomic::{AtomicU8, Ordering}, Arc}, collections::VecDeque, time::Instant};

use egui::ImageData;
use image::{RgbImage, GrayImage, Luma};

use crate::utils;

pub struct PipelineOutput {
    pub frame: ImageData,
    pub fps: usize,
    pub processing_time: f64
}

pub struct Pipeline {
    receiver: Receiver<PipelineOutput>
}

impl Pipeline {
    pub fn new(index: u32, camera_width: u32, camera_height: u32, setting_threshold: Arc<AtomicU8>, setting_block_size: Arc<AtomicU8>) -> Self {
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
                .resolution(camera_width, camera_height).unwrap().start();
            if let Err(e) = image_iterator {
                eprintln!("could retrieve data from camera: {}", e);
                std::process::exit(2);
            }

            let mut images = VecDeque::new();
            // Exponential Moving Average
            let mut ema_init = false;
            let ema_n = 400f32;
            let ema_alpha = 2.0f32 / (ema_n + 1f32);
            let mut ema = vec![0f32; (camera_width * camera_height) as usize];

            // use camera
            let image_iterator = image_iterator.unwrap();
            for image in image_iterator {
                let time = Instant::now();
                let image = RgbImage::from_raw(image.width(), image.height(), image.into_raw()).unwrap();

                let frame_to_display = if let Some((_, _)) = images.front() {
                    
                    for (i, pixel) in image.pixels().enumerate() {
                        let new_value = utils::_rgb_pixel_to_luma_pixel(pixel).0[0] as f32;
                        let delta = new_value - ema[i];
                        ema[i] += ema_alpha * delta;
                    }

                    let ema_image = utils::_f32_image_to_gray_image(&ema, image.width(), image.height());
                    //utils::_gray_image_to_image_data(&ema_image)

                    //let new_image = utils::_rgb_image_diff(last_image, &image);
                    //utils::_rgb_image_to_image_data(&new_image, false)
                    let mut new_image = utils::_gray_image_diff(&ema_image, &utils::_rgb_image_to_gray_image(&image));
                    
                    /*
                    let block_size = setting_block_size.load(Ordering::Relaxed) as u32;
                    let block_width = new_image.width() / block_size;
                    let block_height = new_image.height() / block_size;
                    let mut new_image_x_smash = GrayImage::new(block_width, new_image.height());
                    for y in 0..new_image.height() {
                        for x_block in 0..block_width {
                            let mut block_sum = 0f32;
                            for x_offset in 0..block_size {
                                let x = x_block + x_offset;
                                let pixel = new_image.get_pixel(x, y);
                                block_sum += pixel.0[0] as f32;
                            }
                            new_image_x_smash.put_pixel(x_block, y, Luma([((block_sum / block_size as f32) as u8)]));
                        }
                    }
                    let mut new_image_y_smash = GrayImage::new(block_width, block_height);
                    for y_block in 0..new_image.height() {
                        for x_block in 0..block_width {
                            let mut block_sum = 0f32;
                            for y_offset in 0..block_size {
                                let y = y_block + y_offset;
                                let pixel = new_image_x_smash.get_pixel(x_block, y);
                                block_sum += pixel.0[0] as f32;
                            }
                            new_image_y_smash.put_pixel(x_block, y_block, Luma([((block_sum / block_size as f32) as u8)]));
                        }
                    }
                    */

                    let threshold = setting_threshold.load(Ordering::Relaxed);
                    new_image.pixels_mut().for_each(|p| {
                        if p.0[0] < threshold {
                            p.0[0] = 0;
                        }
                        else {
                            p.0[0] = 255;
                        }
                    });

                    utils::_gray_image_to_image_data(&new_image)
                }
                else {
                    if !ema_init {
                        ema_init = true;

                        for (i, pixel) in image.pixels().enumerate() {
                            ema[i] = utils::_rgb_pixel_to_luma_pixel(pixel).0[0] as f32;
                        }
                    }

                    let new_image = utils::_f32_image_to_gray_image(&ema, image.width(), image.height());
                    utils::_gray_image_to_image_data(&new_image)
                };

                images.push_front((Instant::now(), image));

                images.retain(|(time, _)| time.elapsed().as_secs_f64() < 1.0);

                let pipeline_output = PipelineOutput {
                    frame: frame_to_display,
                    fps: images.len(),
                    processing_time: time.elapsed().as_secs_f64()
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