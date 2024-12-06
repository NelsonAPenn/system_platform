use std::io::{Read, Write};
use system_platform::i2cdev::I2cDev;

fn main() {
    let mut i2cdev = I2cDev::new("/dev/i2c-1\0", 0x41).unwrap();
    let mut buf = [0xd4];

    i2cdev.write_all(&buf).unwrap();

    i2cdev.read_exact(&mut buf).unwrap();

    println!("got reading {}", buf[0]);
}
