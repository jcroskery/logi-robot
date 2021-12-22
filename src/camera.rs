use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::Duration;

use opencv::{
	prelude::*,
	videoio,
    imgcodecs,
    types::{VectorOfi32, VectorOfu8}
};

pub fn start_camera(sender: Sender<serde_json::Value>, timer: Arc<howlong::HighResolutionTimer>) {
    std::thread::spawn(move || {
        if let Ok(mut cam) = videoio::VideoCapture::new(0, videoio::CAP_ANY) {
            if let Ok(opened) = videoio::VideoCapture::is_opened(&cam) {
                if opened {
                    loop {
                        std::thread::sleep(Duration::from_millis(10));
                        let mut frame = Mat::default();
		                cam.read(&mut frame).expect("Failed to read frame");
                        let mut jpg = VectorOfu8::new();
                        imgcodecs::imencode(".jpg", &frame, &mut jpg, &VectorOfi32::new()).expect("Failed to save image");
                        let encoded_data = base64::encode(jpg);
                        sender.send(serde_json::json!({
                            "response": "camera",
                            "time": timer.elapsed().as_nanos() as u64,
                            "image": encoded_data
                        })).unwrap();
                    }
                }
            }
        }
    });
}
