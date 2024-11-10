// #![cfg_attr(not(test), no_std)]
use core::arch::asm;
pub mod error;
pub mod i2c;
pub mod syscall_number;
pub mod i2cdev;

use error::Error;

pub type FileDescriptor = i32;

pub const STDIN: FileDescriptor = 0;
pub const STDOUT: FileDescriptor = 1;
pub const STDERR: FileDescriptor = 2;

const AT_FDCWD: i32 = -100;

#[repr(i32)]
pub enum OpenFlags {
    ReadOnly = 0,
    WriteOnly = 1,
    ReadWrite = 2,
}

pub fn open(path: &str, flags: OpenFlags) -> Result<FileDescriptor, Error> {
    let fd;
    let ptr: *const u8 = path.as_ptr();
    unsafe {
        asm!(
            "svc 0",
            inout("x0") 0 /* dir fd, if not absolute */ => fd,
            in("x1") ptr,
            in("x2") flags as i32,
            in("x3") 0, /* mode */
            in("w8") syscall_number::OPEN_AT
        )
    };
    if fd > 0 {
        Ok(fd)
    } else {
        Err((-fd).into())
    }
}

pub fn close(fd: FileDescriptor) -> Result<(), Error> {
    let retval: i32;
    unsafe {
        asm!(
            "svc 0",
            inout("x0") fd => retval,
            in("w8") syscall_number::CLOSE,
        )
    };
    if retval < 0 {
        Err((-retval).into())
    } else {
        Ok(())
    }
}

pub fn write(fd: FileDescriptor, bytes: &[u8]) -> Result<i32, Error> {
    let bytes_written: i32;
    let ptr: *const u8 = bytes.as_ptr();
    unsafe {
        asm!(
            "svc 0",
            inout("w0") fd => bytes_written,
            in("x1") ptr,
            in("x2") bytes.len(),
            in("w8") syscall_number::WRITE
        )
    };
    if bytes_written < 0 {
        Err((-bytes_written).into())
    } else {
        Ok(bytes_written)
    }
}

pub fn read(fd: FileDescriptor, bytes: &mut [u8]) -> Result<i32, Error> {
    let bytes_read: i32;
    let ptr: *mut u8 = bytes.as_mut_ptr();
    unsafe {
        asm!(
            "svc 0",
            inout("w0") fd => bytes_read,
            in("x1") ptr,
            in("x2") bytes.len(),
            in("w8") syscall_number::READ
        )
    };
    if bytes_read < 0 {
        Err((-bytes_read).into())
    } else {
        Ok(bytes_read)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let fd = open("/dev/i2c-1\0", OpenFlags::ReadWrite);
        println!("{}", fd);
        let ret = close(fd);
        println!("ret {}", ret);
    }
}
