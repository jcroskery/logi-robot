use rppal::gpio::{Gpio, Mode};
use rppal::pwm::{Pwm, Channel, Polarity};
use serde_json::Value;
use servos::ServoTrait;

use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::convert::TryFrom;

mod ultrasonic;
mod infrared;
mod stepper;
mod motor;
mod gyro;
mod servos;
mod camera;

const DIRECTIONPINS: &[u8] = &[20, 21, 13, 26];

const LEDPIN: u8 = 9;
const ULTRASONICPIN: u8 = 10;
const INFRAREDPIN: u8 = 11;

struct Server {
    ws_sender: Arc<Mutex<ws::Sender>>,
    individual_client_receiver: Arc<Mutex<Receiver<ws::Message>>>,
    to_infrared_sender: Sender<(servos::ServoTrait, usize)>,
    to_ultrasonic_sender: Sender<(servos::ServoTrait, usize)>,
    to_led_sender: Sender<(servos::ServoTrait, usize)>,
    to_stepper_sender: Sender<i32>,
    to_motor_sender: Sender<Vec<i32>>
}

impl Server {
    fn respond_to_message(&mut self, msg: ws::Message) -> Result<(), ()> {
        let msg_text = msg.into_text().ok().ok_or(())?;
        let json_message = serde_json::from_str(&msg_text).ok().ok_or(())?;
        if let Value::Object(map) = json_message {
            let request = map.get("request").ok_or(())?;
            match request {
                Value::String(request_string) => {
                    match request_string.as_str() {
                        "stepper" => {
                            let direction = map.get("direction").ok_or(())?.as_i64().ok_or(())? as i32;
                            (&self.to_stepper_sender).send(direction).ok().ok_or(())?;
                        },
                        "motor" => {
                            let drive = map.get("drive").ok_or(())?.as_array().ok_or(())?;
                            let converted_drive: Vec<_> = drive.iter().filter_map(|value| {
                                Some(value.as_i64()? as i32)
                            }).collect();
                            if converted_drive.len() == 2 {
                                (&self.to_motor_sender).send(converted_drive).ok().ok_or(())?;
                            } else {
                                return Err(());
                            }
                        },
                        "servo" => {
                            let chain_name = map.get("chain").ok_or(())?;
                            let sender = match chain_name {
                                Value::String(string) => {
                                    match string.as_str() {
                                        "infrared" => &self.to_infrared_sender,
                                        "led" => &self.to_led_sender,
                                        "ultrasonic" => &self.to_ultrasonic_sender,
                                        _ => return Err(())
                                    }
                                },
                                _ => { return Err(()) }
                            };
                            let module_position = map.get("module").ok_or(())?.as_u64().ok_or(())? as usize;
                            sender.send((servos::ServoTrait::try_from(map)?, module_position)).ok().ok_or(())?;
                        }
                        _ => {
                            return Err(());
                        }
                    }
                    Ok(())
                }
                _ => {
                    return Err(())
                }
            }
        } else {
            Err(())
        }
    }
}

impl ws::Handler for Server {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        let individual_client_receiver = self.individual_client_receiver.clone();
        let ws_sender = self.ws_sender.clone();
        std::thread::spawn(move || {
            loop {
                if let Ok(msg) = individual_client_receiver.lock().unwrap().recv() {
                    ws_sender.lock().unwrap().send(msg).unwrap();
                }
            }
        });
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let message_response = self.respond_to_message(msg);
        if let Err(_) = message_response {
            println!("Internal Error");
        }
        Ok(())
    }
}

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

    let mut to_client_senders: Arc<Mutex<Vec<Sender<ws::Message>>>> = Arc::new(Mutex::new(vec![]));

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
    //to_stepper_sender.send(90);

    servos::ServoChain::start_receive_data_thread(infrared_chain, to_infrared_receiver);
    servos::ServoChain::start_receive_data_thread(ultrasonic_chain, to_ultrasonic_receiver);
    servos::ServoChain::start_receive_data_thread(led_chain, to_led_receiver);
    //to_infrared_sender.send((servos::ServoTrait::LIM(true), 0));

    motor::init_motor(pwm, direction_pins, to_client_message_sender.clone(), to_motor_receiver, timer.clone());
    //to_motor_sender.send(vec![100, 100]).unwrap();

    camera::start_camera(to_client_message_sender.clone(), timer.clone());
    
    let to_client_senders_clone = to_client_senders.clone();
    std::thread::spawn(move || {
        loop {
            if let Ok(received_message) = to_client_message_receiver.recv() {
                //println!("JSON: {}", received_message);
                let mut unlocked_client_senders = to_client_senders_clone.lock().unwrap();
                let mut senders_to_remove = vec![];
                let mut i = 0;
                for client in unlocked_client_senders.iter() {
                    if let Err(_) = client.send(ws::Message::Text(serde_json::to_string(&received_message).unwrap_or(String::new()))) {
                        senders_to_remove.push(i);
                    }
                    i+=1;
                }
                i = 0;
                for sender in senders_to_remove {
                    unlocked_client_senders.remove(sender - i);
                    i+=1;
                }
            }
        }
    });

    if let Err(error) = ws::listen("0.0.0.0:6455", |ws_sender| {
        let (individual_client_sender, individual_client_receiver) = channel();
        to_client_senders.lock().unwrap().push(individual_client_sender);
        Server {
            ws_sender: Arc::new(Mutex::new(ws_sender)),
            individual_client_receiver: Arc::new(Mutex::new(individual_client_receiver)),
            to_infrared_sender: to_infrared_sender.clone(),
            to_ultrasonic_sender: to_ultrasonic_sender.clone(),
            to_led_sender: to_led_sender.clone(),
            to_stepper_sender: to_stepper_sender.clone(),
            to_motor_sender: to_motor_sender.clone()
        }
    }) {
        println!("WebSocket error: {:?}", error);
    }
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
