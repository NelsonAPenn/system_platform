use crate::{open, close, read, write, OpenFlags, FileDescriptor, i2c::i2c_slave};
use embedded_hal::i2c;
use std::io::Error;

pub struct I2cDev
{
    fd: FileDescriptor
}

impl I2cDev{
    pub fn new(devname: &str, address: u8) -> Result<Self, Error>
    {
        let fd = crate::open(devname, OpenFlags::ReadWrite).map_err(Error::from_raw_os_error)?;
        i2c_slave(fd, address as u32).map_err(Error::from_raw_os_error)?;
        Ok(Self {
            fd
        })
    }
}

impl std::io::Read for I2cDev
{

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>
    {
        read(self.fd, buf).map_err(Error::from_raw_os_error)
    }
}

impl std::io::Write for I2cDev
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>
    {
        write(self.fd, buf).map_err(Error::from_raw_os_error)
    }

    fn flush(&mut self) -> Result<(), std::io::Error>
    {
        Ok(())
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
