use rppal::gpio::{InputPin, Gpio, Level, Trigger};

use std::sync::mpsc::{Sender, channel};
use std::time::Duration;
use std::sync::Arc;

const INFRAREDPIN: u8 = 5;

pub fn init_infrared(gpio: Gpio, channel: Sender<serde_json::Value>, 
    timer: Arc<howlong::HighResolutionTimer>) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_millis(50));
            let infrared_pin = gpio.clone().get(INFRAREDPIN).unwrap().into_input();
            if let Ok(infrared) = infrared(infrared_pin) {
                channel.send(serde_json::json!({
                    "response": "infrared",
                    "infrared": infrared,
                    "time": timer.elapsed().as_nanos() as u64
                }));
            }
        }
    });
}

fn infrared (mut infrared_pin: InputPin) -> Result<bool, ()> {
    let (sender, receiver) = channel();
    infrared_pin.set_async_interrupt(Trigger::Both, move |level| {
        sender.send(level == Level::High);
    }).unwrap();
    return receiver.recv().ok().ok_or(());
}