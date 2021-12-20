use rppal::gpio::Gpio;
use serde_json::Value;

use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod ultrasonic;
mod motor;
mod gyro;
mod camera;

struct Server {
    ws_sender: Arc<Mutex<ws::Sender>>,
    individual_client_receiver: Arc<Mutex<Receiver<ws::Message>>>,
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
                        _ => {}
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
    let timer = Arc::new(howlong::HighResolutionTimer::new());
    let gpio = Gpio::new().unwrap();

    let (to_client_message_sender, to_client_message_receiver) = channel();
    
    let (to_motor_sender, to_motor_receiver) = channel();

    let to_client_senders: Arc<Mutex<Vec<Sender<ws::Message>>>> = Arc::new(Mutex::new(vec![]));

    
    ultrasonic::init_ultrasonic(gpio.clone(), to_client_message_sender.clone(), timer.clone());

    gyro::init_gyro(to_client_message_sender.clone(), timer.clone());

    motor::init_motor(gpio.clone(), to_client_message_sender.clone(), to_motor_receiver, timer.clone());

    camera::start_camera(to_client_message_sender.clone(), timer.clone());
    
    let to_client_senders_clone = to_client_senders.clone();
    std::thread::spawn(move || {
        loop {
            if let Ok(received_message) = to_client_message_receiver.recv() {
                println!("JSON: {}", received_message);
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

    
    to_motor_sender.send(vec![100, -100]).expect("Failed to start motors.");
    spin_sleep::sleep(Duration::from_millis(5000));
    println!("Finished sleep. Exiting.");
    to_motor_sender.send(vec![100, -100]).expect("Failed to stop motors.");

    if let Err(error) = ws::listen("0.0.0.0:6455", |ws_sender| {
        let (individual_client_sender, individual_client_receiver) = channel();
        to_client_senders.lock().unwrap().push(individual_client_sender);
        Server {
            ws_sender: Arc::new(Mutex::new(ws_sender)),
            individual_client_receiver: Arc::new(Mutex::new(individual_client_receiver)),
            to_motor_sender: to_motor_sender.clone()
        }
    }) {
        println!("WebSocket error: {:?}", error);
    }
}
