use crate::{open, close, read, write, OpenFlags, FileDescriptor, error::Error, i2c::i2c_slave};
use embedded_hal::i2c;

pub struct I2cDev
{
    fd: FileDescriptor
}

impl I2cDev{
    pub fn new(devname: &str, address: u8) -> Result<Self, Error>
    {
        let fd = crate::open(devname, OpenFlags::ReadWrite)?;
        i2c_slave(fd, address as u32)?;
        Ok(Self {
            fd
        })
    }
}

/* would be std::io::Read if not no_std */
impl I2cDev
{
    pub fn write(&mut self, buf: &[u8]) -> Result<usize, Error>
    {
        write(self.fd, buf)
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>
    {
        read(self.fd, buf)
    }
}

/* would be automatically handled if not no_std */
impl I2cDev
{
    pub fn write_exact(&mut self, buf: &[u8]) -> Result<(), Error>
    {
        let count = self.write(buf)?;
        if count != buf.len()
        {
            Err(Error::IoError)
        }
        else
        {
            Ok(())
        }

    }

    pub fn read_exact(&mut self, buf: &mut[u8]) -> Result<(), Error>
    {
        let count = self.read(buf)?;
        if count != buf.len()
        {
            Err(Error::IoError)
        }
        else
        {
            Ok(())
        }

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
