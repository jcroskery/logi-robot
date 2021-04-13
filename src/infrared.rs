use tokio::time::{self, Duration};
use rppal::gpio::{InputPin, Gpio};

const INFRAREDPIN: u8 = 15;

pub async fn init_infrared_pin(gpio: Gpio) {
    let mut interval = time::interval(Duration::from_millis(50));
    loop {
        interval.tick().await;
        println!("{}", infrared(gpio.get(INFRAREDPIN).unwrap().into_input()).await);
    }
}

async fn infrared (infrared_pin: InputPin) -> bool {
    return infrared_pin.is_high();
}