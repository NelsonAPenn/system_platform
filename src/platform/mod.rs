pub mod i2c;
mod syscall_macro;
mod syscall_number;
use core::arch::asm;
use core::result::{Result, Result::Err, Result::Ok};

use syscall_macro::syscall;

pub type FileDescriptor = i32;
pub type RawOsError = i32;

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

pub fn open(path: &str, flags: OpenFlags) -> Result<FileDescriptor, RawOsError> {
    let ptr: *const u8 = path.as_ptr();
    let fd = syscall!(
        syscall_number::OPEN_AT,
        0, /* fd of directory if not absolute*/
        ptr,
        flags as i32,
        0
    );
    if fd >= 0 {
        Ok(fd)
    } else {
        Err((-fd).into())
    }
}

pub fn close(fd: FileDescriptor) -> Result<(), RawOsError> {
    let retval = syscall!(syscall_number::CLOSE, fd);
    if retval < 0 {
        Err((-retval).into())
    } else {
        Ok(())
    }
}

pub fn write(fd: FileDescriptor, bytes: &[u8]) -> Result<usize, RawOsError> {
    let ptr: *const u8 = bytes.as_ptr();
    let bytes_written = syscall!(syscall_number::WRITE, fd, ptr, bytes.len());
    if bytes_written < 0 {
        Err((-bytes_written).into())
    } else {
        Ok(bytes_written as usize)
    }
}

pub fn read(fd: FileDescriptor, bytes: &mut [u8]) -> Result<usize, RawOsError> {
    let ptr: *mut u8 = bytes.as_mut_ptr();
    let bytes_read = syscall!(syscall_number::READ, fd, ptr, bytes.len());
    if bytes_read < 0 {
        Err((-bytes_read).into())
    } else {
        Ok(bytes_read as usize)
    }
}

pub fn exit(code: usize) -> ! {
    let _retval = syscall!(syscall_number::EXIT_GROUP, code);
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let fd = open("/dev/i2c-1\0", OpenFlags::ReadWrite).unwrap();
        println!("{}", fd);
        close(fd).unwrap();
    }

    #[test]
    fn stdout() {
        let fd = STDOUT;
        write(fd, b"Yaaargh!").unwrap();
    }

    #[test]
    fn echo() {
        let fd = STDIN;
        println!("Type 3 characters:");
        let mut buf = [0; 3];
        read(fd, &mut buf).unwrap();
        println!("You typed {}", std::str::from_utf8(&buf).unwrap());
    }
}
