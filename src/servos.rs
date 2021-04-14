use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level, Mode};

use std::time::Duration;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::channel;
use std::convert::TryInto;
use std::cmp::{max, min};

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

fn calculate_checksum(bytes: Vec<u8>, module: u8) -> u8 {
    let mut sum: u16 = bytes.iter().map(|u8byte| { *u8byte as u16 }).sum();
    sum += sum >> 8;
    sum += sum << 4;
    sum &= 0xf0;
    return (sum as u8) + module;
}

pub fn send_bytes(gpio: Gpio, pin_number: u8, bytes: Vec<u8>, module: u8) {
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
    fn new(gpio: Gpio, pin_number: u8, module_position: u8, bytes: Arc<Mutex<Vec<u8>>>) -> Self where Self: Sized;
    fn get_bytes(&self) -> Vec<u8>;
    fn set_bytes(&mut self, bytes: Vec<u8>);
    fn get_gpio(&self) -> Gpio;
    fn get_pin_number(&self) -> u8;
    fn get_module_position(&self) -> u8;
    fn set_colour(&mut self, colour: (u8, u8, u8));
    fn update(&mut self) -> bool;
    fn get_colour(&self) -> (u8, u8, u8);
    fn set_lim(&mut self, lim: bool) {}
    fn set_pos(&mut self, motor_position: i32) {}
    fn get_pos(&self) -> Option<i32> { None }
    fn get_type(&self) -> ServoType;
    fn send_and_receive(&mut self, bytes: Vec<u8>) -> u8 {
        std::thread::sleep(Duration::from_millis(10));
        send_bytes(self.get_gpio(), self.get_pin_number(), bytes.clone(), self.get_module_position());
        println!("Sent bytes {:?} to module {} on pin {}.", bytes, self.get_module_position(), self.get_pin_number());
        let received_byte = receive_byte(self.get_gpio(), self.get_pin_number());
        println!("Received byte {} from module {} on pin {} in response to {:?}.", received_byte, self.get_module_position(), self.get_pin_number(), bytes);
        return received_byte;
    }
    fn init(&mut self) -> bool {
        self.send_wakeup() && self.send_type_check()
    }
    fn try_send_and_receive(&mut self, desired_value: Option<u8>, undesired_value: Option<u8>) -> Option<u8> {
        let bytes = self.get_bytes();
        for _ in 0..5 {
            println!("Sending {:?} message for module {} on pin {}.", bytes, self.get_module_position(), self.get_pin_number());
            let byte = self.send_and_receive(bytes.clone());
            if desired_value.unwrap_or(!byte) == byte || undesired_value.unwrap_or(byte) != byte { 
                return Some(byte); 
            }
        }
        None
    }
    fn send_wakeup(&mut self) -> bool {
        let mut bytes = self.get_bytes();
        for i in self.get_module_position()..4 {
            bytes[i as usize] = 0xfe;
        }
        self.set_bytes(bytes.clone());
        self.try_send_and_receive(Some(0xfe), None).is_some()
    }
    fn send_type_check(&mut self) -> bool {
        let mut bytes = self.get_bytes();
        bytes[self.get_module_position() as usize] = 0xfc;
        self.set_bytes(bytes.clone());
        let correct_type_response = if self.get_type() == ServoType::MOTOR { 0x01 } else { 0x02 };
        self.try_send_and_receive(Some(correct_type_response), None).is_some()
    }
}

struct Led {
    gpio: Gpio,
    pin_number: u8,
    module_position: u8,
    colour: (u8, u8, u8),
    bytes: Arc<Mutex<Vec<u8>>>
}

impl Led {
    fn update_colour_bit_1(&mut self) -> bool {
        let mut bytes = self.get_bytes();
        bytes[self.get_module_position() as usize] = (self.colour.1 >> 3) + self.colour.0;
        self.set_bytes(bytes.clone());
        self.try_send_and_receive(Some(0x02), None).is_some()
    }

    fn update_colour_bit_2(&mut self) -> bool {
        let mut bytes = self.get_bytes();
        bytes[self.get_module_position() as usize] = 0x40 | self.colour.2;
        self.set_bytes(bytes.clone());
        self.try_send_and_receive(Some(0x02), None).is_some()
    }
}

impl Servo for Led {
    fn new(gpio: Gpio, pin_number: u8, module_position: u8, bytes: Arc<Mutex<Vec<u8>>>) -> Self {
        Led {
            gpio,
            pin_number,
            module_position,
            colour: (0, 0, 7),
            bytes
        }
    }

    fn get_type(&self) -> ServoType { ServoType::LED }

    fn set_colour(&mut self, colour: (u8, u8, u8)) {
        self.colour = colour;
    }

    fn set_bytes(&mut self, bytes: Vec<u8>) {
        *self.bytes.lock().unwrap() = bytes;
    }

    fn get_colour(&self) -> (u8, u8, u8) { self.colour }
    fn get_gpio(&self) -> Gpio { self.gpio.clone() }
    fn get_pin_number(&self) -> u8 { self.pin_number }
    fn get_module_position(&self) -> u8 { self.module_position }
    fn get_bytes(&self) -> Vec<u8> { self.bytes.lock().unwrap().clone() }

    fn update(&mut self) -> bool {
        self.update_colour_bit_1() && self.update_colour_bit_2()
    }
}

struct Motor {
    gpio: Gpio, 
    pin_number: u8,
    module_position: u8,
    colour: (u8, u8, u8),
    motor_position: i32,
    lim: bool,
    bytes: Arc<Mutex<Vec<u8>>>
}

impl Motor {
    fn update_colour(&mut self) -> bool {
        let mut bytes = self.get_bytes();
        let colour_bits = (self.colour.2 << 2) + (self.colour.1 << 1) + self.colour.0;
        bytes[self.get_module_position() as usize] = 0xf0 | colour_bits;
        self.set_bytes(bytes.clone());
        self.try_send_and_receive(None, Some(0x00)).is_some()
    }

    fn update_pos(&mut self) -> bool {
        if !self.lim {
            let mut bytes = self.get_bytes();
            let position_byte = ((self.motor_position + 90) as f64 / 180.0 * 208.0) as u8 + 0x18;
            bytes[self.get_module_position() as usize] = position_byte;
            self.set_bytes(bytes.clone());
            self.try_send_and_receive(None, Some(0x00)).is_some()
        } else {
            true
        }
    }
    fn update_lim(&mut self) -> bool {
        if self.lim {
            let mut bytes = self.get_bytes();
            bytes[self.get_module_position() as usize] = 0xfa;
            self.set_bytes(bytes.clone());
            if let Some(byte) = self.try_send_and_receive(None, Some(0x00)) {
                let motor_position = (((byte - 0x18) as f64) / 208.0 * 180.0 - 90.0) as i32;
                let clamped_motor_position = max(min(motor_position, 90), -90);
                self.motor_position = clamped_motor_position;
                true
            } else {
                false
            }
        } else {
            true
        }
    }
}

impl Servo for Motor {
    fn new(gpio: Gpio, pin_number: u8, module_position: u8, bytes: Arc<Mutex<Vec<u8>>>) -> Self {
        Motor {
            gpio,
            pin_number,
            module_position,
            colour: (0, 0, 1),
            motor_position: 0,
            lim: false,
            bytes
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
    fn get_bytes(&self) -> Vec<u8> { self.bytes.lock().unwrap().clone() }

    fn set_lim(&mut self, lim: bool) {
        self.lim = lim;
    }

    fn set_bytes(&mut self, bytes: Vec<u8>) {
        println!("Setting current bytes {:?} to new bytes {:?}.", self.bytes, bytes);
        *self.bytes.lock().unwrap() = bytes;
        println!("Bytes are now {:?}.", self.bytes);
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

    fn update(&mut self) -> bool {
        self.update_colour() && self.update_pos() && self.update_lim()
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
        let bytes = Arc::new(Mutex::new(vec![0, 0, 0, 0]));
        for i in 0..servo_types.len() {
            let module_position = i.try_into().unwrap();
            if servo_types[i] == ServoType::LED {
                servos.push(Box::new(Led::new(gpio.clone(), pin_number, module_position, bytes.clone())));
            } else if servo_types[i] == ServoType::MOTOR {
                servos.push(Box::new(Motor::new(gpio.clone(), pin_number, module_position, bytes.clone())));
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
        println!("Successfully initialized servo chain on pin {}.", self.pin_number);
    }
    fn try_update(&mut self) -> bool {
        for servo in &mut self.servos {
            if !servo.update() {
                return false;  
            }
        }
        true
    }
    pub fn set_lim(&mut self, lim: bool, module_position: usize) {
        self.servos[module_position].set_lim(lim);
    }
    pub fn set_colour(&mut self, colour: (u8, u8, u8), module_position: usize) {
        self.servos[module_position].set_colour(colour);
    }
    pub fn set_pos(&mut self, pos: i32, module_position: usize) {
        self.servos[module_position].set_pos(pos);
    }
    pub fn update(&mut self) {
        loop {
            if self.try_update() { 
                break; 
            } else {
                self.init();
            }
        }
        println!("Successfully updated servo chain on pin {}.", self.pin_number);
    }
}

