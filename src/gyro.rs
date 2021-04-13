use rppal::i2c::I2c;
use std::time::Duration;

const GYROADDRESS: u16 = 0x68;
const POWERREGISTER: u8 = 0x6b;
const GYROREGISTER: u8 = 0x3b;

pub fn gyro() -> Vec<f64> {
    let mut i2c = I2c::new().unwrap();
    let mut buffer = [0; 14];
    i2c.set_slave_address(GYROADDRESS).unwrap();
    i2c.block_write(POWERREGISTER, &[0x01]).unwrap();
    spin_sleep::sleep(Duration::from_millis(10));
    i2c.block_read(GYROREGISTER, &mut buffer).unwrap();
    let mut gyro_readings = vec![];
    for i in 0..7 {
        if i == 3 {continue;}
        let mut bits: u16 = (buffer[i] as u16) << 8 + (buffer[i + 1] as u16);
        let mut combined_bits = 0.0;
        println!("Bits {} (before conv): {}", i, bits);
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
        println!("Bits {} (after conv): {}", i, bits);
    }
    println!("{:?}", buffer);
    println!("{:?}", gyro_readings);
    return gyro_readings;
}