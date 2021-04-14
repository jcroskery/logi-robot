use rppal::i2c::I2c;

use std::time::Duration;
use std::sync::mpsc::{Sender, channel};

const GYROADDRESS: u16 = 0x68;
const POWERREGISTER: u8 = 0x6b;
const GYROREGISTER: u8 = 0x3b;

pub fn init_gyro(channel: Sender<serde_json::Value>) {
    std::thread::spawn(move || {
        let mut gyro = Gyro::new();
        loop {
            std::thread::sleep(Duration::from_millis(10));
            let (gyro_readings, time_elapsed) = gyro.read();
            channel.send(serde_json::json!({
                "gyroscope": gyro_readings,
                "d_time": time_elapsed
            })).unwrap();
        }
    });
}

struct Gyro {
    i2c: I2c,
    timer: howlong::HighResolutionTimer,
    time: u128
}

impl Gyro {
    pub fn new() -> Self {
        let mut i2c = I2c::new().unwrap();
        let mut timer = howlong::HighResolutionTimer::new();
        i2c.set_slave_address(GYROADDRESS).unwrap();
        i2c.block_write(POWERREGISTER, &[0x01]).unwrap();
        Gyro { i2c, timer, time: 0 }
    }
    pub fn read(&mut self) -> (Vec<f64>, u128) {
        let mut buffer = [0; 14];
        self.i2c.block_read(GYROREGISTER, &mut buffer).unwrap();
        let unadjusted_time = self.timer.elapsed().as_nanos();
        let timer = unadjusted_time - self.time;
        self.time = unadjusted_time;
        let mut gyro_readings = vec![];
        for i in 0..7 {
            if i == 3 {continue;}
            let mut bits: u16 = ((buffer[i * 2] as u16) << 8) + (buffer[i * 2 + 1] as u16);
            let mut combined_bits = 0.0;
            if (bits.leading_ones() > 0) {
                combined_bits = -((!bits + 1) as f64);
            } else {
                combined_bits = bits as f64;
            }
            if i > 3 {
                gyro_readings.push(combined_bits / 131.072);
            } else {
                gyro_readings.push(combined_bits / 16384.0);
            }
        }
        (gyro_readings, timer)
    }
}
