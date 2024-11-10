const EPERM: i32 = 1;
const ENOENT: i32 = 2;
const ESRCH: i32 = 3;
const EINTR: i32 = 4;
const EIO: i32 = 5;
const ENXIO: i32 = 6;
const E2BIG: i32 = 7;
const ENOEXEC: i32 = 8;
const EBADF: i32 = 9;

const EREMOTEIO: i32 = 121;

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum Error {
    OperationNotPermitted = EPERM,
    NoSuchFileOrDirectory = ENOENT,
    NoSuchProcess = ESRCH,
    InterruptedSystemCall = EINTR,
    IoError = EIO,
    NoSuchDeviceOrAddress = ENXIO,
    ArgumentListTooLong = E2BIG,
    ExecFormatError = ENOEXEC,
    BadFileDescriptor = EBADF,

    RemoteIoError = EREMOTEIO,
}

impl From<i32> for Error {
    fn from(value: i32) -> Self {
        unsafe { *(&value as *const i32 as *const Self) }
    }
}
