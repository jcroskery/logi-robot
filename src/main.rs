use rppal::gpio::Gpio;

mod ultrasonic;

#[tokio::main]
async fn main() {
    let gpio = Gpio::new().unwrap();

    let handle = tokio::spawn(async {
        ultrasonic::init_ultrasonic_pins(gpio).await;
    });

    println!("Hello, world!");
    handle.await.unwrap();
}
