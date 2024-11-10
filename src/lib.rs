#![cfg_attr(not(test), no_std)]
use core::arch::asm;
pub mod syscall_number;

pub type FileDescriptor = i32;

pub const STDIN: FileDescriptor = 0;
pub const STDOUT: FileDescriptor = 1;
pub const STDERR: FileDescriptor = 2;

const AT_FDCWD: i32 = -100;

#[repr(i32)]
pub enum OpenFlags
{
    ReadOnly = 0,
    WriteOnly = 1,
    ReadWrite = 2,
}



pub fn open(path: &str, flags: OpenFlags) -> FileDescriptor
{
    let fd;
    let ptr: *const u8 = path.as_ptr();
    unsafe {asm!(
        "svc 0",
        inout("x0") 0 /* dir fd, if not absolute */ => fd,
        in("x1") ptr,
        in("x2") flags as i32,
        in("x3") 0, /* mode */
    )};
    fd
}

pub fn write(fd: FileDescriptor, bytes: &[u8]) -> i32
{
    let bytes_written;
    let ptr: *const u8 = bytes.as_ptr();
    unsafe {asm!(
        "svc 0",
        inout("w0") fd => bytes_written,
        in("x1") ptr,
        in("x2") bytes.len(),
        in("w8") syscall_number::WRITE
    )};
    bytes_written
}

pub fn read(fd: FileDescriptor, bytes: &mut [u8]) -> i32
{
    let bytes_read;
    let ptr: *mut u8 = bytes.as_mut_ptr();
    unsafe {asm!(
        "svc 0",
        inout("w0") fd => bytes_read,
        in("x1") ptr,
        in("x2") bytes.len(),
        in("w8") syscall_number::READ
    )};
    bytes_read
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let fd = open("/dev/i2c-1", OpenFlags::ReadWrite);
        // println!("{}", fd);
        let mut buf = [0; 8];
        let count = read(STDIN, &mut buf);
        let outbuf = format!("count: {}, read: {}", count, std::str::from_utf8(&buf).unwrap());
        write(STDOUT, outbuf.as_bytes());
    }
}
