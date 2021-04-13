use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level};
use tokio::time::Duration;
use rppal::pwm::{Pwm, Channel, Polarity};

use std::convert::TryInto;
const DIRECTIONPINS: &[u8] = &[13, 26, 20, 21];

pub fn drive(gpio: Gpio, pwm: &[Pwm], speeds: &[i32]) {
    let map = |pin_number: &u8| { gpio.get(*pin_number).unwrap().into_output()};
    let mut enable_pins: Vec<_> = DIRECTIONPINS.iter().map(map).collect();
    for i in enable_pins.iter() {
        println!("Enable pin: {}", i.is_set_high());
    }
    for i in 0..2 {
        pwm[i].set_duty_cycle(speeds[i].abs() as f64 / 100.0).unwrap();
        //speed_pins[i].set_high();
        if speeds[i] > 0 {
            enable_pins[i * 2].set_high();
            enable_pins[i * 2 + 1].set_low();
        } else {
            enable_pins[i * 2].set_low();
            enable_pins[i * 2 + 1].set_high();
        }
    }
    println!("Enable pin 2:");
    spin_sleep::sleep(Duration::from_millis(5000));
}
