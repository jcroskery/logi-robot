use tokio::time::{self, Duration};
use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level};

const TRIGGERPIN: u8 =  14;
const ECHOPIN: u8 = 15;

pub async fn init_ultrasonic_pins(gpio: Gpio) {
    let mut trigger_pin = gpio.get(TRIGGERPIN).unwrap().into_output();
    let mut echo_pin = gpio.get(ECHOPIN).unwrap().into_input();
    ultrasonic(&mut trigger_pin, &mut echo_pin).await;
}

async fn ultrasonic (&mut trigger_pin: OutputPin, &mut echo_pin: InputPin) {
    let mut interval = time::interval(Duration::from_millis(50));
    interval.tick().await;
    trigger_pin.set_high();
    spin_sleep::sleep(Duration::from_millis(1000));
    trigger_pin.set_low();
    let mut timer = howlong::HighResolutionTimer::new();
    println!("This happens");
    echo_pin.set_async_interrupt(Trigger::Both, move |level| {
        println!("Happens");
        if level == Level::High {
            timer.start();
        } else {
            println!("#{}\n", timer.elapsed().subsec_nanos());
        }
        ultrasonic(trigger_pin, echo_pin);
    }).unwrap();
}