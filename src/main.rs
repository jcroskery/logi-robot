use rppal::gpio::Gpio;
use tokio::time::Duration;

mod ultrasonic;

#[tokio::main]
async fn main() {
    let gpio = Gpio::new().unwrap();

    tokio::spawn(async {
        println!("Hello, us!");
        ultrasonic::init_ultrasonic_pins(gpio).await;
    });

    spin_sleep::sleep(Duration::from_millis(1000));
    println!("Hello, you!");
}
