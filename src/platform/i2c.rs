use crate::platform::{syscall_macro::syscall, syscall_number, FileDescriptor, RawOsError};
use core::arch::asm;
use core::result::{Result, Result::Err, Result::Ok};
use ioctl_consts::*;

#[allow(dead_code)]
mod ioctl_consts {
    pub const I2C_RETRIES: usize = 0x0701;
    pub const I2C_TIMEOUT: usize = 0x0702;
    pub const I2C_SLAVE: usize = 0x0703;
    pub const I2C_SLAVE_FORCE: usize = 0x0706;
    pub const I2C_TENBIT: usize = 0x0704;
    pub const I2C_FUNCS: usize = 0x0705;
    pub const I2C_RDWR: usize = 0x0707;
    pub const I2C_PEC: usize = 0x0708;
    pub const I2C_SMBUS: usize = 0x0720;
}

/*
#[repr(C)]
pub struct I2cSmbusIoctlData
{
    pub u8 read_write;
    pub u8 command;
    pub u32 size;
    // union
}
*/

pub fn i2c_slave(fd: FileDescriptor, address: u32) -> Result<(), RawOsError> {
    let retval: i32;
    syscall!(syscall_number::IOCTL, retval, fd, I2C_SLAVE, address);
    if retval < 0 {
        Err((-retval).into())
    } else {
        Ok(())
    }
}
