pub mod i2c;
mod syscall_number;
use core::arch::asm;

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
    let fd;
    let ptr: *const u8 = path.as_ptr();
    unsafe {
        #[cfg(target_arch = "arm")]
        {
            asm!(
                "svc 0",
                inout("r0") 0 /* dir fd, if not absolute */ => fd,
                in("r1") ptr,
                in("r2") flags as i32,
                in("r3") 0, /* mode */
                in("r8") syscall_number::OPEN_AT
            )
        }
        #[cfg(target_arch = "aarch64")]
        {
            asm!(
                "svc 0",
                inout("x0") 0 /* dir fd, if not absolute */ => fd,
                in("x1") ptr,
                in("x2") flags as i32,
                in("x3") 0, /* mode */
                in("w8") syscall_number::OPEN_AT
            )
        }
    };
    if fd >= 0 {
        Ok(fd)
    } else {
        Err((-fd).into())
    }
}

pub fn close(fd: FileDescriptor) -> Result<(), RawOsError> {
    let retval: i32;
    unsafe {
        #[cfg(target_arch = "arm")]
        {
            asm!(
                "svc 0",
                inout("r0") fd => retval,
                in("r8") syscall_number::CLOSE,
            )
        }
        #[cfg(target_arch = "aarch64")]
        {
            asm!(
                "svc 0",
                inout("x0") fd => retval,
                in("w8") syscall_number::CLOSE,
            )
        }
    };
    if retval < 0 {
        Err((-retval).into())
    } else {
        Ok(())
    }
}

pub fn write(fd: FileDescriptor, bytes: &[u8]) -> Result<usize, RawOsError> {
    let bytes_written: i32;
    let ptr: *const u8 = bytes.as_ptr();
    unsafe {
        #[cfg(target_arch = "arm")]
        {
            asm!(
                "svc 0",
                inout("r0") fd => bytes_written,
                in("r1") ptr,
                in("r2") bytes.len(),
                in("r8") syscall_number::WRITE
            )
        }
        #[cfg(target_arch = "aarch64")]
        {
            asm!(
                "svc 0",
                inout("w0") fd => bytes_written,
                in("x1") ptr,
                in("x2") bytes.len(),
                in("w8") syscall_number::WRITE
            )
        }
    };
    if bytes_written < 0 {
        Err((-bytes_written).into())
    } else {
        Ok(bytes_written as usize)
    }
}

pub fn read(fd: FileDescriptor, bytes: &mut [u8]) -> Result<usize, RawOsError> {
    let bytes_read: i32;
    let ptr: *mut u8 = bytes.as_mut_ptr();
    unsafe {
        #[cfg(target_arch = "arm")]
        {
            asm!(
                "svc 0",
                inout("r0") fd => bytes_read,
                in("r1") ptr,
                in("r2") bytes.len(),
                in("r8") syscall_number::READ
            )
        }
        #[cfg(target_arch = "aarch64")]
        {
            asm!(
                "svc 0",
                inout("w0") fd => bytes_read,
                in("x1") ptr,
                in("x2") bytes.len(),
                in("w8") syscall_number::READ
            )
        }
    };
    if bytes_read < 0 {
        Err((-bytes_read).into())
    } else {
        Ok(bytes_read as usize)
    }
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
}
