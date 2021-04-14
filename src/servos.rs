use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level, Mode};
use std::time::Duration;

use std::sync::mpsc::channel;
use std::convert::TryInto;

const BITDELAY: u64 = 417;

fn send_byte(pin: &mut OutputPin, byte: u8) {
    let sleeper = spin_sleep::SpinSleeper::default();
    pin.set_low();
    sleeper.sleep(Duration::from_micros(BITDELAY));
    for i in 0..8 {
        if (byte >> i) & 0x01 == 1 {
            pin.set_high();
        } else {
            pin.set_low();
        }
        sleeper.sleep(Duration::from_micros(BITDELAY));
    }
    pin.set_high();
    sleeper.sleep(Duration::from_micros(BITDELAY * 2));
}

fn calculate_checksum(bytes: &[u8], module: u8) -> u8 {
    let mut sum: u16 = bytes.iter().map(|u8byte| { *u8byte as u16 }).sum();
    sum += sum >> 8;
    sum += sum << 4;
    sum &= 0xf0;
    return (sum as u8) + module;
}

pub fn send_bytes(gpio: Gpio, pin_number: u8, bytes: &[u8], module: u8) {
    let mut pin = gpio.get(pin_number).unwrap().into_output();
    send_byte(&mut pin, 0xff);
    for i in 0..4 {
        send_byte(&mut pin, bytes[i]);
    }
    send_byte(&mut pin, calculate_checksum(bytes, module));
}

pub fn receive_byte(gpio: Gpio, pin_number: u8) -> u8 {
    let mut pin = gpio.get(pin_number).unwrap().into_input();
    let mut received_byte = 0;
    let mut timer = howlong::HighResolutionTimer::new();
    timer.stop();
    let (pin_sender, receiver) = channel();
    let timeout_sender = pin_sender.clone();
    pin.set_async_interrupt(Trigger::Both, move |level| {
        if level == Level::High {
            pin_sender.send(true).unwrap();
        } else {
            pin_sender.send(false).unwrap();
        }
    });
    println!("Receiving byte on pin {}.", pin_number);
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(50));
        timeout_sender.send(false);
        timeout_sender.send(false);
    });
    for i in 0..8 {
        if receiver.recv().unwrap() {
            timer.start();
        } else {
            return 0;
        }
        if !receiver.recv().unwrap() {
            if(timer.elapsed().as_micros() > 400) {
                received_byte |= 1 << i;
            }
            timer.stop();
        } else {
            return 0;
        }
    }
    return received_byte;
}

#[derive(PartialEq)]
pub enum ServoType {
    LED,
    MOTOR,
    EMPTY
}

trait Servo {
    fn new(gpio: Gpio, pin_number: u8, module_position: u8) -> Self where Self: Sized;
    fn get_gpio(&self) -> Gpio;
    fn get_pin_number(&self) -> u8;
    fn get_module_position(&self) -> u8;
    fn set_colour(&mut self, colour: (u8, u8, u8));
    fn get_colour(&self) -> (u8, u8, u8);
    fn set_lim(&mut self, lim: bool) {}
    fn set_pos(&mut self, motor_position: i32) {}
    fn get_pos(&self) -> Option<i32> { None }
    fn get_type(&self) -> ServoType;
    fn send_and_receive(&mut self, bytes: &[u8]) -> u8 {
        send_bytes(self.get_gpio(), self.get_pin_number(), bytes, self.get_module_position());
        println!("Sent bytes {:?} to module {} on pin {}.", bytes, self.get_module_position(), self.get_pin_number());
        let received_byte = receive_byte(self.get_gpio(), self.get_pin_number());
        println!("Received byte {} from module {} on pin {} in response to {:?}.", received_byte, self.get_module_position(), self.get_pin_number(), bytes);
        return received_byte;
    }
    fn init(&mut self) -> bool {
        let bytes = &mut [0xfe; 4];
        for i in 0..self.get_module_position() {
            bytes[i as usize] = 0xfc;
        }
        println!("Sending initialization message for module {} on pin {}.", self.get_module_position(), self.get_pin_number());
        if self.send_and_receive(bytes) == 0xfe {
            bytes[self.get_module_position() as usize] = 0xfc;
            println!("Sending type message for module {} on pin {}.", self.get_module_position(), self.get_pin_number());
            let correct_type_response = if self.get_type() == ServoType::MOTOR { 0x02 } else { 0x01 };
            if self.send_and_receive(bytes) == correct_type_response {
                return true;
            }
        }
        println!("Initialization of module {} on pin {} failed", self.get_module_position(), self.get_pin_number());
        false
    }
}

struct Led {
    gpio: Gpio,
    pin_number: u8,
    module_position: u8,
    colour: (u8, u8, u8)
}

impl Servo for Led {
    fn new(gpio: Gpio, pin_number: u8, module_position: u8) -> Self {
        Led {
            gpio,
            pin_number,
            module_position,
            colour: (0, 0, 7)
        }
    }

    fn get_type(&self) -> ServoType { ServoType::LED }

    fn set_colour(&mut self, colour: (u8, u8, u8)) {
        self.colour = colour;
    }

    fn get_colour(&self) -> (u8, u8, u8) { self.colour }
    fn get_gpio(&self) -> Gpio { self.gpio.clone() }
    fn get_pin_number(&self) -> u8 { self.pin_number }
    fn get_module_position(&self) -> u8 { self.module_position }
}

struct Motor {
    gpio: Gpio, 
    pin_number: u8,
    module_position: u8,
    colour: (u8, u8, u8),
    motor_position: i32,
    lim: bool
}

impl Servo for Motor {
    fn new(gpio: Gpio, pin_number: u8, module_position: u8) -> Self {
        Motor {
            gpio,
            pin_number,
            module_position,
            colour: (0, 0, 7),
            motor_position: 0,
            lim: false
        }
    }
    
    fn get_type(&self) -> ServoType { ServoType::MOTOR }

    fn set_colour(&mut self, colour: (u8, u8, u8)) {
        self.colour = colour;
    }

    fn get_colour(&self) -> (u8, u8, u8) { self.colour }
    fn get_gpio(&self) -> Gpio { self.gpio.clone() }
    fn get_pin_number(&self) -> u8 { self.pin_number }
    fn get_module_position(&self) -> u8 { self.module_position }

    fn set_lim(&mut self, lim: bool) {
        self.lim = lim;
    }

    fn set_pos(&mut self, motor_position: i32) {
        if self.lim == false {
            self.motor_position = motor_position;
        }
    }

    fn get_pos(&self) -> Option<i32> {
        if self.lim == true {
            //Todo: Update lim reading
        }
        Some(self.motor_position)
    }
}

pub struct ServoChain {
    gpio: Gpio,
    pin_number: u8,
    servos: Vec<Box<dyn Servo>>
}

impl ServoChain {
    pub fn new(gpio: Gpio, pin_number: u8, servo_types: Vec<ServoType>) -> Self {
        let mut servos: Vec<Box<dyn Servo>> = vec![];
        for i in 0..servo_types.len() {
            let module_position = i.try_into().unwrap();
            if servo_types[i] == ServoType::LED {
                servos.push(Box::new(Led::new(gpio.clone(), pin_number, module_position)));
            } else if servo_types[i] == ServoType::MOTOR {
                servos.push(Box::new(Motor::new(gpio.clone(), pin_number, module_position)));
            }
        }
        let mut servo_chain = ServoChain {
            gpio,
            pin_number,
            servos
        };
        servo_chain.init();
        servo_chain
    }
    fn try_init(&mut self) -> bool {
        for servo in &mut self.servos {
            if !servo.init() {
                return false;  
            }
        }
        true
    }
    pub fn init(&mut self) {
        loop {
            if self.try_init() { break; }
        }
        println!("Successfully initialized servo chain");
    }
}

