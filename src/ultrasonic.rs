use tokio::time::{self, Duration};
use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level};

const TRIGGERPIN: i32 = 14;
const ECHOPIN: i32 = 15;

async fn init_ultrasonic_pins(gpio: Gpio) {
    let mut triggerPin = gpio.get(TRIGGERPIN)?.into_output();
    let mut echoPin = gpio.get(ECHOPIN)?.into_input();
    ultrasonic(triggerPin, echoPin);
}

async fn ultrasonic (mut triggerPin: OutputPin, mut echoPin: InputPin) {
    let mut interval = time::interval(Duration::from_millis(50));
    interval.tick().await;
    triggerPin.set_high();
    spin_sleep::sleep(Duration::from_millis(10));
    triggerPin.set_low();
    let timer = howlong::HighResolutionTimer::new();
    echoPin.set_async_interrupt(Trigger::Both, |level| {
        if (level == Level::High) {
            timer.start();
        } else {
            println!("%l\n", timer.elapsed().subsec_nanos());
        }
    });
}