use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level};
use tokio::time::Duration;

const PINS: &[u8] = &[4, 17, 27, 22];

pub async fn init_stepper_pins(gpio: Gpio) {
    loop {
        let mut pins = [gpio.get(PINS[0]).unwrap().into_output(), gpio.get(PINS[1]).unwrap().into_output(), gpio.get(PINS[2]).unwrap().into_output(), gpio.get(PINS[3]).unwrap().into_output()];
        println!("{}\n", stepper(&mut pins).await);
    }
}

async fn stepper(pins: &mut [OutputPin]) -> i32 {
    pins[0].set_high();
    for i in 1..4 {
        pins[i].set_low();
    }
    for i in 0..((90.0 / 360.0 * 512.0 / 64.0 * 63.68395) as i32) {
        for j in 0..8 {
            let flip = j / 2 + 2 * j % 2;
            if pins[j].is_set_high() { pins[j].set_low(); } else { pins[j].set_high(); }
            spin_sleep::sleep(Duration::from_micros(700));
        }
    }
    return 0;
}
