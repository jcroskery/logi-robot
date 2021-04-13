use std::time::Duration;
use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level};

use std::sync::mpsc::channel;

//use std::sync::{Arc, Mutex};

const TRIGGERPIN: u8 = 14;
const ECHOPIN: u8 = 15;

pub async fn init_ultrasonic_pins(gpio: Gpio) {
    loop {
        println!("{}\n", ultrasonic(gpio.get(TRIGGERPIN).unwrap().into_output(), gpio.get(ECHOPIN).unwrap().into_input()).await);
    }
}

async fn ultrasonic (mut trigger_pin: OutputPin, mut echo_pin: InputPin) -> f32 {
    trigger_pin.set_high();
    spin_sleep::sleep(Duration::from_millis(10));
    trigger_pin.set_low();
    let mut timer = howlong::HighResolutionTimer::new();
    timer.stop();
    let (sender, receiver) = channel();
    echo_pin.set_async_interrupt(Trigger::Both, move |level| {
        if level == Level::High {
            timer.start();
        } else {
            sender.send(timer.elapsed().subsec_nanos() as f32 / 1000.0 * 0.017).unwrap();
        }
    }).unwrap();
    return receiver.recv().unwrap();
}