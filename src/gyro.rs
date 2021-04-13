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
    for i in 0..7 {
        if i == 3 {continue;}
        let mut bits: u16 = (buffer[i] as u16) << 8 + (buffer[i + 1] as u16);
        if (bits.leading_ones() > 0) {
            bits = !bits + 1;
        }
        println!("{}", bits);
    }
    println!("{:?}", buffer);
}