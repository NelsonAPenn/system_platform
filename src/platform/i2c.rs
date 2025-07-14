use crate::platform::{syscall_number, FileDescriptor, RawOsError};
use core::arch::asm;
use ioctl_consts::*;

#[allow(dead_code)]
mod ioctl_consts {
    pub const I2C_RETRIES: u64 = 0x0701;
    pub const I2C_TIMEOUT: u64 = 0x0702;
    pub const I2C_SLAVE: u64 = 0x0703;
    pub const I2C_SLAVE_FORCE: u64 = 0x0706;
    pub const I2C_TENBIT: u64 = 0x0704;
    pub const I2C_FUNCS: u64 = 0x0705;
    pub const I2C_RDWR: u64 = 0x0707;
    pub const I2C_PEC: u64 = 0x0708;
    pub const I2C_SMBUS: u64 = 0x0720;
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
    unsafe {
        #[cfg(target_arch = "arm")]
        {
            asm!(
                "svc 0",
                inout("r0") fd => retval,
                in("r1") I2C_SLAVE as u32,
                in("r2") address,
                in("r7") syscall_number::IOCTL
            )
        }
        #[cfg(target_arch = "aarch64")]
        {
            asm!(
                "svc 0",
                inout("x0") fd => retval,
                in("x1") I2C_SLAVE,
                in("x2") address,
                in("w8") syscall_number::IOCTL
            )
        }
        #[cfg(target_arch = "x86_64")]
        {
            asm!(
                "syscall",
                in("rdi") fd,
                in("rsi") I2C_SLAVE,
                in("rdx") address,
                inout("rax") syscall_number::IOCTL => retval,
            )
        }
    };
    if retval < 0 {
        Err((-retval).into())
    } else {
        Ok(())
    }
}
