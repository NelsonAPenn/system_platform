pub use syscall_no::*;

#[cfg(target_arch = "aarch64")]
mod syscall_no {
    pub const IOCTL: u32 = 29;
    pub const OPEN_AT: u32 = 56;
    pub const CLOSE: u32 = 57;
    pub const READ: u32 = 63;
    pub const WRITE: u32 = 64;
    pub const EXIT: u32 = 93;
    pub const EXIT_GROUP: u32 = 94;
}

#[cfg(target_arch = "arm")]
mod syscall_no {
    pub const IOCTL: u32 = 54;
    pub const OPEN_AT: u32 = 322;
    pub const CLOSE: u32 = 6;
    pub const READ: u32 = 3;
    pub const WRITE: u32 = 4;
    pub const EXIT: u32 = 1;
    pub const EXIT_GROUP: u32 = 248;
}

#[cfg(target_arch = "x86_64")]
mod syscall_no {
    pub const IOCTL: u32 = 64;
    pub const OPEN_AT: u32 = 257;
    pub const CLOSE: u32 = 3;
    pub const READ: u32 = 0;
    pub const WRITE: u32 = 1;
    pub const EXIT: u32 = 66;
    pub const EXIT_GROUP: u32 = 231;
}
