use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{Pwm, Channel, Polarity};

use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver};

const DIRECTIONPINS: &[u8] = &[26, 4, 6, 21];

pub fn init_motor(gpio: Gpio, sender: Sender<serde_json::Value>, receiver: Receiver<Vec<i32>>, 
    timer: Arc<howlong::HighResolutionTimer>) {
    std::thread::spawn(move || {
        let mut direction_pins = DIRECTIONPINS.iter().map(|pin_number: &u8| { gpio.get(*pin_number).unwrap().into_output()}).collect();
        let mut pwm = vec![Pwm::with_frequency(Channel::Pwm1,100.0, 0.0,
            Polarity::Normal, true).unwrap(), 
            Pwm::with_frequency(Channel::Pwm0,100.0, 0.0,
            Polarity::Normal, true).unwrap()];
        loop {
            if let Ok(speeds) = receiver.recv() {
                println!("Changed speed");
                sender.send(serde_json::json!({
                    "response": "motor",
                    "time": timer.elapsed().as_nanos() as u64
                })).unwrap();
                drive(&mut pwm, &mut direction_pins, speeds);
            }
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
