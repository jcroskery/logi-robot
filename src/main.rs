use rppal::gpio::Gpio;
use rppal::pwm::{Pwm, Channel, Polarity};
use std::time::Duration;

mod ultrasonic;
mod infrared;
mod stepper;
mod motor;

const ENABLEPINS: &[u8] = &[18, 19];
const DIRECTIONPINS: &[u8] = &[13, 26, 20, 21];

fn main() {
    let gpio = Gpio::new().unwrap();
    let mut enable_pins: Vec<_> = ENABLEPINS.iter().map(|pin_number: &u8| { gpio.get(*pin_number).unwrap().into_output()}).collect();
    let mut direction_pins: Vec<_> = DIRECTIONPINS.iter().map(|pin_number: &u8| { gpio.get(*pin_number).unwrap().into_output()}).collect();
    /*
    let mut pwm = [Pwm::with_frequency(Channel::Pwm0,100.0, 0.0,
            Polarity::Normal, true).unwrap(), 
            Pwm::with_frequency(Channel::Pwm1,100.0, 0.0,
            Polarity::Normal, true).unwrap()];
            
    pwm[0].set_reset_on_drop(false);
    pwm[1].set_reset_on_drop(false);
    pwm[0].set_duty_cycle(1.0).unwrap();
    pwm[1].set_duty_cycle(1.0).unwrap();
    for i in 0..10000000 {
        for j in 0..100000000 {
            if (i > j + 1000000001) {
                panic!("WHWHWHWHWHWHATATATATATAT");
            }
        }
    }
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
    motor::drive(gpio.clone(), &mut enable_pins, &mut direction_pins, &[100, 100]);
    spin_sleep::sleep(Duration::from_millis(5000));
    println!("Finished sleep. Exiting.");
    motor::drive(gpio, &mut enable_pins, &mut direction_pins, &[0, 0]);
}
