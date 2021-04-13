use rppal::gpio::Gpio;
use tokio::time::Duration;

mod ultrasonic;
mod infrared;

#[tokio::main]
async fn main() {
    let gpio = Gpio::new().unwrap();

    let ultrasonic_gpio = gpio.clone();
    tokio::spawn(async {
        ultrasonic::init_ultrasonic_pins(ultrasonic_gpio).await;
    });

    
    tokio::spawn(async {
        infrared::init_infrared_pin(gpio).await;
    });

    spin_sleep::sleep(Duration::from_millis(1000));
    println!("Finished sleep. Exiting.");
}
