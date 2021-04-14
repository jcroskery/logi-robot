use rppal::gpio::{OutputPin, InputPin, Gpio, Trigger, Level, Mode};
use std::time::Duration;

use std::sync::mpsc::channel;

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
    println!("Test");
    send_byte(&mut pin, 0xff);
    for i in 0..4 {
        send_byte(&mut pin, bytes[i]);
    }
    println!("{}", calculate_checksum(bytes, module));
    send_byte(&mut pin, calculate_checksum(bytes, module));
}

pub fn receive_byte(gpio: Gpio, pin_number: u8) -> u8 {
    let mut pin = gpio.get(pin_number).unwrap().into_input();
    let mut received_byte = 0;
    let mut timer = howlong::HighResolutionTimer::new();
    timer.stop();
    let (sender, receiver) = channel();
    /*
    pin.set_async_interrupt(Trigger::Both, move |level| {
        println!("Received message");
        if level == Level::High {
            sender.send(true).unwrap();
        } else {
            sender.send(false).unwrap();
        }
    }).unwrap();
    */
    for i in 0..8 {
        println!("iter {}", i);
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

