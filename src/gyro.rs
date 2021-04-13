use rppal::i2c::I2c;

const GYROADDRESS: u16 = 0x68;
const POWERREGISTER: u8 = 0x6b;
const GYROREGISTER: u8 = 0x3b;

pub fn gyro() {
    let mut i2c = I2c::new().unwrap();
    let mut buffer = [0; 14];
    i2c.set_slave_address(GYROADDRESS).unwrap();
    i2c.block_write(POWERREGISTER, &[0x01]).unwrap();
    i2c.block_read(GYROREGISTER, &mut buffer).unwrap();
    println!("{:?}", buffer);
}