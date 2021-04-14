use std::time::Duration;
use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level};

use std::sync::mpsc::{Sender, channel};

const TRIGGERPIN: u8 = 14;
const ECHOPIN: u8 = 15;

pub fn init_ultrasonic_pins(gpio: Gpio, channel: Sender<serde_json::Value>) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_millis(50));
            let trigger_pin = gpio.get(TRIGGERPIN).unwrap().into_output();
            let echo_pin =  gpio.get(ECHOPIN).unwrap().into_input();
            channel.send(serde_json::json!({
                "ultrasonic": ultrasonic(trigger_pin, echo_pin)
            }));
        }
    });
}

fn ultrasonic (mut trigger_pin: OutputPin, mut echo_pin: InputPin) -> f32 {
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