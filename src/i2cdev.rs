use crate::{open, close, read, write, OpenFlags, FileDescriptor, error::Error, i2c::i2c_slave};
use embedded_hal::i2c;

pub struct I2cDev
{
    fd: FileDescriptor
}

impl I2cDev
{
    pub fn new(devname: &str, address: u8) -> Result<Self, Error>
    {
        let fd = crate::open(devname, OpenFlags::ReadWrite)?;
        i2c_slave(fd, address as u32)?;
        Ok(Self {
            fd
        })
    }

    pub fn write(&self, buf: &[u8]) -> Result<i32, Error>
    {
        write(self.fd, buf)
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<i32, Error>
    {
        read(self.fd, buf)
    }
}

impl Drop for I2cDev
{
    fn drop(&mut self)
    {
        close(self.fd).unwrap();
    }
}


/*
impl i2c::I2c for I2c
{

    fn write(&mut self, 


}
*/
