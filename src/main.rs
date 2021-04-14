use rppal::gpio::{Gpio, Mode};
use rppal::pwm::{Pwm, Channel, Polarity};
use servos::ServoChain;
use std::time::Duration;

mod ultrasonic;
mod infrared;
mod stepper;
mod motor;
mod gyro;
mod servos;

const DIRECTIONPINS: &[u8] = &[20, 21, 13, 26];

const LEDPIN: u8 = 9;
const ULTRASONICPIN: u8 = 10;
const INFRAREDPIN: u8 = 11;

fn main() {
    let gpio = Gpio::new().unwrap();
    //let mut direction_pins: Vec<_> = DIRECTIONPINS.iter().map(|pin_number: &u8| { gpio.get(*pin_number).unwrap().into_output()}).collect();
    /*
    let mut pwm = [Pwm::with_frequency(Channel::Pwm0,100.0, 0.0,
            Polarity::Normal, true).unwrap(), 
            Pwm::with_frequency(Channel::Pwm1,100.0, 0.0,
            Polarity::Normal, true).unwrap()];
    */
    //let mut servo_pins = [gpio.get(LEDPIN).unwrap().into_io(Mode::Output), 
    //    gpio.get(ULTRASONICPIN).unwrap().into_io(Mode::Output), 
    //    gpio.get(INFRAREDPIN).unwrap().into_io(Mode::Output)];
    //servos::send_bytes(gpio.clone(), LEDPIN, &[254, 0, 0, 0], 0);
    //println!("{}", servos::receive_byte(gpio.clone(), LEDPIN));
    let infrared_chain = servos::ServoChain::new(gpio, ULTRASONICPIN, 
        vec![servos::ServoType::MOTOR, servos::ServoType::MOTOR]);
    
    //let ultrasonic_gpio = gpio.clone();
    //ultrasonic::init_ultrasonic_pins(gpio);
    /*

    let infrared_gpio = gpio.clone();
    tokio::spawn(async {
        infrared::init_infrared_pin(infrared_gpio).await;
    });

    tokio::spawn(async {
        stepper::init_stepper_pins(gpio).await;
    });
    motor::drive(&mut pwm, &mut direction_pins, &[100, -100]);
    spin_sleep::sleep(Duration::from_millis(5000));
    println!("Finished sleep. Exiting.");
    motor::drive(&mut pwm, &mut direction_pins, &[0, 0]);
    loop {
        gyro::gyro();
    }
    */
}
