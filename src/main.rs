use rppal::gpio::Gpio;

mod ultrasonic;

#[tokio::main]
async fn main() {
    let gpio = Gpio::new().unwrap();

    tokio::spawn(async {
        ultrasonic::init_ultrasonic_pins(gpio);
    });

    println!("Hello, world!");
}
