use crate::syscall_number;
use crate::FileDescriptor;
use crate::error::Error;
use core::arch::asm;

const I2C_RETRIES: u64 = 0x0701;
const I2C_TIMEOUT: u64 = 0x0702;
const I2C_SLAVE: u64 = 0x0703;
const I2C_SLAVE_FORCE: u64 = 0x0706;
const I2C_TENBIT: u64 = 0x0704;
const I2C_FUNCS: u64 = 0x0705;
const I2C_RDWR: u64 = 0x0707;
const I2C_PEC: u64 = 0x0708;
const I2C_SMBUS: u64 = 0x0720;

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

pub fn i2c_slave(fd: FileDescriptor, address: u32) -> Result<(), Error> {
    let retval: i32;
    unsafe {
        asm!(
            "svc 0",
            inout("x0") fd => retval,
            in("x1") I2C_SLAVE,
            in("x2") address,
            in("w8") syscall_number::IOCTL
        )
    };
    if retval < 0
    {
        Err((-retval).into())
    }
    else
    {
        Ok(())
    }
}


