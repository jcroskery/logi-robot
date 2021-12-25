use rppal::gpio::{Gpio, InputPin, Level, OutputPin, Trigger};

use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::time::Duration;

const TRIGGERPIN: u8 = 14;
const ECHOPIN: u8 = 15;

const NUMBEROFSTOREDVALUES: usize = 20;

pub fn init_ultrasonic(
    gpio: Gpio,
    channel: Sender<serde_json::Value>,
    timer: Arc<howlong::HighResolutionTimer>,
) {
    std::thread::spawn(move || {
        let mut past_ultrasonic_values = vec![-1.0; NUMBEROFSTOREDVALUES];
        let mut i = 0;
        loop {
            std::thread::sleep(Duration::from_millis(50));
            let trigger_pin = gpio.get(TRIGGERPIN).unwrap().into_output();
            let echo_pin = gpio.get(ECHOPIN).unwrap().into_input();
            let ultrasonic_response = ultrasonic(trigger_pin, echo_pin);
            past_ultrasonic_values[i] = ultrasonic_response;

            let mut cloned_vec = past_ultrasonic_values.clone();
            cloned_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if cloned_vec[0] != -1.0 {
                let median_ultrasonic_response = (cloned_vec[9] + cloned_vec[10]) / 2.0;

                if let Err(error) = channel.send(serde_json::json!({
                    "response": "ultrasonic",
                    "ultrasonic": median_ultrasonic_response,
                    "time": timer.elapsed().as_nanos() as u64
                })) {
                    println!("Error sending ultrasonic data: {}", error);
                };

                if i == 0 {
                    println!("{}", median_ultrasonic_response);
                }
            }
            i += 1;
            if i == 20 {
                i = 0;
            }
        }
    });
}

fn ultrasonic(mut trigger_pin: OutputPin, mut echo_pin: InputPin) -> f32 {
    trigger_pin.set_high();
    spin_sleep::sleep(Duration::from_millis(10));
    trigger_pin.set_low();
    let mut timer = howlong::HighResolutionTimer::new();
    timer.stop();
    let (sender, receiver) = channel();
    let timeout_sender = sender.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(60));
        if let Ok(()) = timeout_sender.send(-1.0) {
            println!("Ultrasonic timeout");
        };
    });
    echo_pin
        .set_async_interrupt(Trigger::Both, move |level| {
            if level == Level::High {
                timer.start();
            } else {
                sender
                    .send(timer.elapsed().subsec_nanos() as f32 / 1000.0 * 0.017)
                    .unwrap();
            }
        })
        .unwrap();
    if let Ok(message) = receiver.recv() {
        if message > 400.0 {
            400.0
        } else if message == -1.0 {
            ultrasonic(trigger_pin, echo_pin) 
        } else {
            message
        }
    } else {
        ultrasonic(trigger_pin, echo_pin)
    }
}
