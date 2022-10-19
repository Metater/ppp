use std::sync::mpsc::Receiver;

use image::{ImageBuffer, Rgb};

pub fn start() -> Receiver<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let res1 = camera_capture::create(0);
        if let Err(e) = res1 {
            eprintln!("could not open camera: {}", e);
            std::process::exit(1);
        }
        
        let res2 = res1.unwrap()
            .resolution(640, 480).unwrap()
            .fps(30.0).unwrap().start();
        if let Err(e) = res2 {
            eprintln!("could retrieve data from camera: {}", e);
            std::process::exit(2);
        }

        let cam = res2.unwrap();
        for frame in cam {
            if sender.send(frame).is_err() {
                break;
            }
        }
    });
    receiver
}