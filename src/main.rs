use rppal::gpio::{Gpio, Mode};
use rppal::pwm::{Pwm, Channel, Polarity};

use std::sync::mpsc::{Sender, channel};
use std::sync::Arc;
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
    let mut timer = Arc::new(howlong::HighResolutionTimer::new());
    let gpio = Gpio::new().unwrap();
    let mut direction_pins: Vec<_> = DIRECTIONPINS.iter().map(|pin_number: &u8| { gpio.get(*pin_number).unwrap().into_output()}).collect();
    let mut pwm = vec![Pwm::with_frequency(Channel::Pwm0,100.0, 0.0,
            Polarity::Normal, true).unwrap(), 
            Pwm::with_frequency(Channel::Pwm1,100.0, 0.0,
            Polarity::Normal, true).unwrap()];
    //let mut servo_pins = [gpio.get(LEDPIN).unwrap().into_io(Mode::Output), 
    //    gpio.get(ULTRASONICPIN).unwrap().into_io(Mode::Output), 
    //    gpio.get(INFRAREDPIN).unwrap().into_io(Mode::Output)];
    //servos::send_bytes(gpio.clone(), LEDPIN, &[254, 0, 0, 0], 0);
    //println!("{}", servos::receive_byte(gpio.clone(), LEDPIN));
    let (to_client_message_sender, to_client_message_receiver) = channel();

    let (to_stepper_sender, to_stepper_receiver) = channel();
    let (to_infrared_sender, to_infrared_receiver) = channel();
    let (to_ultrasonic_sender, to_ultrasonic_receiver) = channel();
    let (to_led_sender, to_led_receiver) = channel();
    let (to_motor_sender, to_motor_receiver) = channel();

    let mut to_client_senders: Vec<Sender<serde_json::Value>> = vec![];

    let mut infrared_chain = servos::ServoChain::new(gpio.clone(), INFRAREDPIN, 
        vec![servos::ServoType::MOTOR, servos::ServoType::MOTOR]);
    let mut ultrasonic_chain = servos::ServoChain::new(gpio.clone(), ULTRASONICPIN, 
        vec![servos::ServoType::MOTOR, servos::ServoType::MOTOR]);
    let mut led_chain = servos::ServoChain::new(gpio.clone(), LEDPIN, 
        vec![servos::ServoType::LED]);
    
    servos::ServoChain::start_get_data_thread(infrared_chain.clone(), to_client_message_sender.clone(), timer.clone());
    servos::ServoChain::start_get_data_thread(ultrasonic_chain.clone(), to_client_message_sender.clone(), timer.clone());


    infrared::init_infrared(gpio.clone(), to_client_message_sender.clone(), timer.clone());
    
    ultrasonic::init_ultrasonic(gpio.clone(), to_client_message_sender.clone(), timer.clone());

    gyro::init_gyro(to_client_message_sender.clone(), timer.clone());
    
    

    stepper::init_stepper(gpio.clone(), to_client_message_sender.clone(), 
        to_stepper_receiver, timer.clone());
    to_stepper_sender.send(90);

    servos::ServoChain::start_receive_data_thread(infrared_chain, to_infrared_receiver);
    servos::ServoChain::start_receive_data_thread(ultrasonic_chain, to_ultrasonic_receiver);
    servos::ServoChain::start_receive_data_thread(led_chain, to_led_receiver);
    to_infrared_sender.send((servos::ServoTrait::LIM(true), 0));

    motor::init_motor(pwm, direction_pins, to_client_message_sender.clone(), to_motor_receiver, timer.clone());
    //to_motor_sender.send(vec![100, 100]).unwrap();
    
    //std::thread::spawn(move || {
        loop {
            let received_message = to_client_message_receiver.recv().unwrap();
            println!("JSON: {}", received_message);
            to_client_senders = to_client_senders.into_iter()
                .filter(|sender| { if let Err(_) = sender.send(received_message.clone()) { false } else { true }}).collect();
        }
    //});
    /* 
    infrared_chain.lock().unwrap().set_lim(true, 0);
    infrared_chain.lock().unwrap().set_pos(90, 1);
    led_chain.lock().unwrap().set_colour((7, 0, 0), 0);
    println!("LIM reading: {}", infrared_chain.lock().unwrap().get_pos(0).unwrap());
    std::thread::sleep(Duration::from_secs(1));
    println!("LIM reading: {}", infrared_chain.lock().unwrap().get_pos(0).unwrap());
    //let ultrasonic_gpio = gpio.clone();
    //ultrasonic::init_ultrasonic_pins(gpio);
    

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
