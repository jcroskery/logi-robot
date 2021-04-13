use tokio::time::{self, Duration};
use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level};

use std::sync::{Arc, Mutex};

const TRIGGERPIN: u8 = 14;
const ECHOPIN: u8 = 15;

pub async fn init_ultrasonic_pins(gpio: Gpio) {
    loop {
        println!("This happens, good");
        ultrasonic(gpio.get(TRIGGERPIN).unwrap().into_output(), gpio.get(ECHOPIN).unwrap().into_input()).await;
    }
}

async fn ultrasonic (mut trigger_pin: OutputPin, mut echo_pin: InputPin) {
    let mut interval = time::interval(Duration::from_millis(50));
    interval.tick().await;
    trigger_pin.set_high();
    spin_sleep::sleep(Duration::from_millis(10));
    trigger_pin.set_low();
    let timer = Arc::new(Mutex::new(howlong::HighResolutionTimer::new()));
    echo_pin.set_async_interrupt(Trigger::Both, move |level| {
        println!("Happens");
        if level == Level::High {
            timer.lock().unwrap().start();
        } else {
            println!("#{}\n", timer.lock().unwrap().elapsed().subsec_nanos());
        }
    }).unwrap();
    interval.tick().await;
}