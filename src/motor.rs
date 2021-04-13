use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level};
use std::time::Duration;
use rppal::pwm::{Pwm, Channel, Polarity};

use std::convert::TryInto;

const DIRECTIONPINS: &[u8] = &[13, 26, 20, 21];

pub fn drive(gpio: Gpio, enable_pins: &mut Vec<OutputPin>, speeds: &[i32]) {
    let mut direction_pins: Vec<_> = DIRECTIONPINS.iter().map(|pin_number: &u8| { gpio.get(*pin_number).unwrap().into_output()}).collect();
    for i in 0..2 {
        enable_pins[i].set_pwm_frequency(100.0, speeds[i].abs() as f64 / 100.0).unwrap();
        //speed_pins[i].set_high();
        if speeds[i] > 0 {
            direction_pins[i * 2].set_high();
            direction_pins[i * 2 + 1].set_low();
        } else {
            direction_pins[i * 2].set_low();
            direction_pins[i * 2 + 1].set_high();
        }
    }
}
