use rscam::{Camera, Config};

use std::sync::Arc;
use std::sync::mpsc::Sender;

pub fn start_camera(sender: Sender<serde_json::Value>, timer: Arc<howlong::HighResolutionTimer>) {
    std::thread::spawn(move || {
        println!("HI");
        let mut camera = Camera::new("/dev/video0").unwrap();

        camera
            .start(&Config {
                interval: (1, 2),
                resolution: (640, 426),
                format: b"MJPG",
                ..Default::default()
            })
            .unwrap();

        let frame = camera.capture().unwrap();

        loop {
            sender
                .send(serde_json::json!({
                    "response": "camera",
                    "time": timer.elapsed().as_nanos() as u64,
                    "frame": base64::encode(&*camera.capture().unwrap())
                }))
                .unwrap();
        }
    });
}