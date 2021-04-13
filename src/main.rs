use rppal::gpio::Gpio;
use tokio::time::Duration;

mod ultrasonic;
mod infrared;
mod stepper;
mod motor;

#[tokio::main]
async fn main() {
    let gpio = Gpio::new().unwrap();

    let ultrasonic_gpio = gpio.clone();
    tokio::spawn(async {
        //ultrasonic::init_ultrasonic_pins(ultrasonic_gpio).await;
    });

    let infrared_gpio = gpio.clone();
    tokio::spawn(async {
        //infrared::init_infrared_pin(infrared_gpio).await;
    });

    tokio::spawn(async {
        //stepper::init_stepper_pins(gpio).await;
    });
    //motor::drive(gpio.clone(), &[100, 100]);
    spin_sleep::sleep(Duration::from_millis(5000));
    //println!("Finished sleep. Exiting.");
    motor::drive(gpio, &[0, 0]);
}
