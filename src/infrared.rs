use rppal::gpio::{InputPin, Gpio, Level, Trigger};

use std::sync::mpsc::{Sender, channel};
use std::time::Duration;

const INFRAREDPIN: u8 = 5;

pub fn init_infrared_pin(gpio: Gpio, channel: Sender<serde_json::Value>) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_millis(50));
            let infrared_pin = gpio.clone().get(INFRAREDPIN).unwrap().into_input();
            channel.send(serde_json::json!({
                "infrared": infrared(infrared_pin)
            }));
        }
    });
}

fn infrared (mut infrared_pin: InputPin) -> bool {
    let (sender, receiver) = channel();
    infrared_pin.set_async_interrupt(Trigger::Both, move |level| {
        sender.send(level == Level::High).unwrap();
    }).unwrap();
    return receiver.recv().unwrap();
}