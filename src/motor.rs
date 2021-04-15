use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level};
use rppal::pwm::{Pwm, Channel, Polarity};

use std::time::Duration;
use std::convert::TryInto;
use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver};

pub fn init_motor(mut pwm: Vec<Pwm>, mut direction_pins: Vec<OutputPin>, sender: Sender<serde_json::Value>, receiver: Receiver<Vec<i32>>, 
    timer: Arc<howlong::HighResolutionTimer>) {
    std::thread::spawn(move || {
        loop {
            let speeds = receiver.recv().unwrap();
            sender.send(serde_json::json!({
                "time": timer.elapsed().as_nanos() as u64
            })).unwrap();
            drive(&mut pwm, &mut direction_pins, speeds);
        }
    });
}

pub fn drive(enable_pins: &mut Vec<Pwm>, direction_pins: &mut Vec<OutputPin>, speeds: Vec<i32>) {
    for i in 0..2 {
        enable_pins[i].set_frequency(100.0, speeds[i].abs() as f64 / 100.0).unwrap();
        if speeds[i] < 0 {
            direction_pins[i * 2].set_high();
            direction_pins[i * 2 + 1].set_low();
        } else {
            direction_pins[i * 2].set_low();
            direction_pins[i * 2 + 1].set_high();
        }
    }
}
