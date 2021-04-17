use rppal::gpio::{OutputPin, Gpio};

use std::time::Duration;
use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver};

const PINS: &[u8] = &[4, 17, 27, 22];
const STEPS: &[usize] = &[1, 0, 2, 1, 3, 2, 0, 3];

pub fn init_stepper(gpio: Gpio, sender: Sender<serde_json::Value>, receiver: Receiver<i32>, 
    timer: Arc<howlong::HighResolutionTimer>) {
    std::thread::spawn(move || {
        loop {
            let mut pins = [gpio.get(PINS[0]).unwrap().into_output(), gpio.get(PINS[1]).unwrap().into_output(), gpio.get(PINS[2]).unwrap().into_output(), gpio.get(PINS[3]).unwrap().into_output()];
            if let Ok(dist) = receiver.recv() {
                sender.send(serde_json::json!({
                    "response": "stepper",
                    "start": true,
                    "time": timer.elapsed().as_nanos() as u64
                })).unwrap();
                stepper(&mut pins, dist);
                sender.send(serde_json::json!({
                    "response": "stepper",
                    "start": false,
                    "time": timer.elapsed().as_nanos() as u64
                })).unwrap();
            }
        }
    });
}

fn stepper(pins: &mut [OutputPin], dist: i32)  {
    pins[0].set_high();
    for i in 1..4 {
        pins[i].set_low();
    }
    for i in 0..((dist.abs() as f32 / 360.0 * 512.0 / 64.0 * 63.68395) as i32) {
        let fore = |j: usize| {
            if pins[STEPS[j]].is_set_high() { pins[STEPS[j]].set_low(); } else { pins[STEPS[j]].set_high(); }
            spin_sleep::sleep(Duration::from_micros(2000));
        };
        if dist < 0 { 
            (0..8).rev().for_each(fore); 
        } else { 
            (0..8).for_each(fore);
        }
    }
    pins[0].set_low();
}
