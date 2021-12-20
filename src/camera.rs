use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::Duration;

use opencv::{
	prelude::*,
	videoio,
    imgcodecs,
    types::VectorOfi32
};

pub fn start_camera(sender: Sender<serde_json::Value>, timer: Arc<howlong::HighResolutionTimer>) {
    std::thread::spawn(move || {
        if let Ok(mut cam) = videoio::VideoCapture::new(0, videoio::CAP_ANY) {
            if let Ok(opened) = videoio::VideoCapture::is_opened(&cam) {
                if opened {
                    std::thread::sleep(Duration::from_secs(5));
                    let mut frame = Mat::default();
		            cam.read(&mut frame).expect("Failed to read frame");
                    imgcodecs::imwrite("test.jpg", &frame, &VectorOfi32::new()).expect("Failed to save image");// /home/pi/logi/test.jpg
                    println!("Saved image from camera");
                }
            }
        }
        
        /* 
        loop {
            sender
                .send(serde_json::json!({
                    "response": "camera",
                    "time": timer.elapsed().as_nanos() as u64,
                    "frame": base64::encode(&*camera.capture().unwrap())
                }))
                .unwrap();
        }
        */
    });
}
