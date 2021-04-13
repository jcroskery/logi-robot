use rppal::gpio::Gpio;

mod ultrasonic;

#[tokio::main]
async fn main() {
    let gpio = Gpio::new().unwrap();

    let handle = tokio::spawn(async {
        println!("Hello, us!");
        ultrasonic::init_ultrasonic_pins(gpio).await;
    });

    println!("Hello, you!");
    handle.await.unwrap();
}
