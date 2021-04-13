use rppal::gpio::{InputPin, Gpio, Level, Trigger};

use std::sync::mpsc::channel;

const INFRAREDPIN: u8 = 15;

pub async fn init_infrared_pin(gpio: Gpio) {
    loop {
        println!("{}", infrared(gpio.get(INFRAREDPIN).unwrap().into_input()).await);
    }
}

async fn infrared (mut infrared_pin: InputPin) -> bool {
    let (sender, receiver) = channel();
    infrared_pin.set_async_interrupt(Trigger::Both, move |level| {
        sender.send(level == Level::High).unwrap();
    }).unwrap();
    return receiver.recv().unwrap();
}