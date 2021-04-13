use rppal::gpio::Gpio;
use rppal::pwm::{Pwm, Channel, Polarity};
use tokio::time::Duration;

mod ultrasonic;
mod infrared;
mod stepper;
mod motor;

fn main() {
    let gpio = Gpio::new().unwrap();
    let pwm = [Pwm::with_frequency(Channel::Pwm0,100.0, 0.0,
            Polarity::Normal, true).unwrap(), 
            Pwm::with_frequency(Channel::Pwm1,100.0, 0.0,
            Polarity::Normal, true).unwrap()];

    /*
    let ultrasonic_gpio = gpio.clone();
    tokio::spawn(async {
        ultrasonic::init_ultrasonic_pins(ultrasonic_gpio).await;
    });

    let infrared_gpio = gpio.clone();
    tokio::spawn(async {
        infrared::init_infrared_pin(infrared_gpio).await;
    });

    tokio::spawn(async {
        stepper::init_stepper_pins(gpio).await;
    });
    */
    motor::drive(gpio.clone(), &pwm, &[100, 100]);
    pwm[0].set_duty_cycle(1.0).unwrap();
    spin_sleep::sleep(Duration::from_millis(5000));
    println!("Finished sleep. Exiting.");
    motor::drive(gpio, &pwm, &[0, 0]);
}
