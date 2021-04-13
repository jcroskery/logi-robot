use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level};
use tokio::time::Duration;

use std::convert::TryInto;

const SPEEDPINS: &[u8] = &[12, 13];
const DIRECTIONPINS: &[u8] = &[19, 26, 20, 21];

pub async fn drive(gpio: Gpio, speeds: &[i32]) {
    let map = |pin_number: &u8| { gpio.get(*pin_number).unwrap().into_output()};
    let mut speed_pins: Vec<_> = SPEEDPINS.iter().map(map).collect();
    let mut enable_pins: Vec<_> = DIRECTIONPINS.iter().map(map).collect();
    for i in 0..2 {
        //speed_pins[i].set_pwm(Duration::from_millis(10), Duration::from_micros((50 * speeds[i].abs()).try_into().unwrap()));
        speed_pins[i].set_high();
        if speeds[i] < 0 {
            enable_pins[i * 2].set_high();
            enable_pins[i * 2 + 1].set_low();
        } else {
            enable_pins[i * 2].set_low();
            enable_pins[i * 2 + 1].set_high();
        }
    }
}
