use crate::platform::i2c::i2c_slave;
use crate::i2cdev::I2cDev;
use std::io::{Read, Write};
use embedded_hal::i2c::{self, I2c, Operation, SevenBitAddress};

#[derive(Debug)]
pub struct Error(std::io::Error);
impl From<std::io::Error> for Error
{
    fn from(err: std::io::Error) -> Self
    {
        Self(err)
    }
}



impl i2c::Error for Error {
    fn kind(&self) -> i2c::ErrorKind {
        i2c::ErrorKind::Other
    }
}

impl i2c::ErrorType for I2cDev {
    type Error = Error;
}

impl I2c<SevenBitAddress> for I2cDev {
    fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
        i2c_slave(self.fd, address as u32).map_err(|err| Error(std::io::Error::from_raw_os_error(err)))?;
        for operation in operations.iter_mut()
        {
            match *operation
            {
                Operation::Read(ref mut buf) => {
                    self.read_exact(buf)?;
                }
                Operation::Write(ref buf) => {
                    self.write_all(buf)?;
                }
            }
        }
        Ok(())
    }
}

/*
impl I2c<TenBitAddress> for I2c0 {
    fn transaction(&mut self, address: u16, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
        // ...
    }
}

*/
