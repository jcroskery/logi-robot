use rppal::gpio::Gpio;

mod ultrasonic;

#[tokio::main]
async fn main() {
    let gpio = Gpio::new().unwrap();

    let mut handle = tokio::spawn(async {
        ultrasonic::init_ultrasonic_pins(gpio).await;
    });

    println!("Hello, world!");
    futures::future::join(handle).await();
}
