use crate::platform::syscall_macro::syscall;

use super::{syscall_number, FileDescriptor, RawOsError};

#[repr(C)]
#[derive(Debug)]
pub struct Gpio {
    pub gpio: u32,
    pub flags: u64, // Is 32 on 32-bit?
    pub label: *const u8,
}

const GPIO_MAX_NAME_SIZE: usize = 32;

#[repr(C)]
#[derive(Debug)]
pub struct GpioChipInfo {
    pub name: [u8; GPIO_MAX_NAME_SIZE],
    pub label: [u8; GPIO_MAX_NAME_SIZE],
    pub lines: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct GpioLineInfo {
    pub name: [u8; GPIO_MAX_NAME_SIZE],
    pub consumer: [u8; GPIO_MAX_NAME_SIZE],
    pub offset: u32,
    pub num_attrs: u32,
    pub flags: u64,
    pub attrs: [GpioLineAttribute; GPIO_LINE_NUM_ATTRS_MAX],
    pub padding: [u32; 4],
}

#[repr(C)]
#[derive(Debug)]
pub struct GpioLineValues {
    pub bits: u64,
    pub mask: u64,
}

const GPIO_LINES_MAX: usize = 64;

#[repr(C)]
#[derive(Debug)]
pub enum GpioLineAttribute {
    Flags {
        id: u32,
        padding: u32,
        flags: u64,
    },
    Values {
        id: u32,
        padding: u32,
        values: u64,
    },
    Debounce {
        id: u32,
        padding: u32,
        debounce_period_us: u32,
    },
}

mod line_attr_id {
    pub const FLAGS: u32 = 1;
    pub const OUTPUT_VALUES: u32 = 2;
    pub const DEBOUNCE: u32 = 3;
}

impl GpioLineAttribute {
    pub fn new_flags(flags: u64) -> Self {
        Self::Flags {
            id: line_attr_id::FLAGS,
            padding: 0,
            flags,
        }
    }

    pub fn new_values(values: u64) -> Self {
        Self::Values {
            id: line_attr_id::OUTPUT_VALUES,
            padding: 0,
            values,
        }
    }

    pub fn new_debounce(debounce_period_us: u32) -> Self {
        Self::Debounce {
            id: line_attr_id::DEBOUNCE,
            padding: 0,
            debounce_period_us,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct GpioLineConfigAttribute {
    pub attr: GpioLineAttribute,
    pub mask: u64,
}
const GPIO_LINE_NUM_ATTRS_MAX: usize = 10;

#[repr(C)]
pub struct GpioLineConfig {
    pub flags: u64,
    pub num_attrs: u32,
    pub padding: [u32; 5],
    pub attrs: [GpioLineConfigAttribute; GPIO_LINE_NUM_ATTRS_MAX],
}

#[repr(C)]
pub struct GpioLineRequest {
    pub offsets: [u32; GPIO_LINES_MAX],
    pub consumer: [u8; GPIO_MAX_NAME_SIZE],
    pub config: GpioLineConfig,
    pub num_lines: u32,
    pub event_buffer_size: u32,
    pub padding: [u32; 5],
    pub fd: FileDescriptor,
}

pub mod line_flag {
    pub const USED: u64 = 1 << 0;
    pub const ACTIVE_LOW: u64 = 1 << 1;
    pub const INPUT: u64 = 1 << 2;
    pub const OUTPUT: u64 = 1 << 3;
    pub const EDGE_RISING: u64 = 1 << 4;
    pub const EDGE_FALLING: u64 = 1 << 5;
    pub const OPEN_DRAIN: u64 = 1 << 6;
    pub const OPEN_SOURCE: u64 = 1 << 7;
    pub const BIAS_PULL_UP: u64 = 1 << 8;
    pub const BIAS_PULL_DOWN: u64 = 1 << 9;
    pub const BIAS_DISABLED: u64 = 1 << 10;
    pub const EVENT_CLOCK_REALTIME: u64 = 1 << 11;
    pub const EVENT_CLOCK_HTE: u64 = 1 << 12;
}

mod ioctl_const {
    pub const GET_LINE: usize = 0x7;
    pub const LINE_GET_VALUES: usize = 0xe;
    pub const LINE_SET_VALUES: usize = 0xf;
    pub const GET_CHIP_INFO: usize = 0x1;
    pub const GET_LINE_INFO: usize = 0x5;
}

pub fn get_line(
    chip_fd: FileDescriptor,
    request: &mut GpioLineRequest,
) -> Result<FileDescriptor, RawOsError> {
    let retval = syscall!(
        syscall_number::IOCTL,
        chip_fd,
        ioctl_const::GET_LINE,
        &request
    );
    if retval < 0 {
        Err((-retval).into())
    } else {
        Ok(request.fd)
    }
}

pub fn get_values(line_fd: FileDescriptor, mask: u64) -> Result<GpioLineValues, RawOsError> {
    let mut gpio_values = GpioLineValues { bits: 0, mask };
    let retval = syscall!(
        syscall_number::IOCTL,
        line_fd,
        ioctl_const::LINE_GET_VALUES,
        &mut gpio_values
    );
    if retval < 0 {
        Err((-retval).into())
    } else {
        Ok(gpio_values)
    }
}

pub fn set_values(line_fd: FileDescriptor, values: &GpioLineValues) -> Result<(), RawOsError> {
    let retval = syscall!(
        syscall_number::IOCTL,
        line_fd,
        ioctl_const::LINE_SET_VALUES,
        values
    );
    if retval < 0 {
        Err((-retval).into())
    } else {
        Ok(())
    }
}

pub fn get_chip_info(chip_fd: FileDescriptor) -> Result<GpioChipInfo, RawOsError> {
    let mut chip_info: GpioChipInfo = unsafe { std::mem::zeroed() };
    let retval = syscall!(
        syscall_number::IOCTL,
        chip_fd,
        ioctl_const::GET_CHIP_INFO,
        &mut chip_info
    );

    if retval < 0 {
        Err((-retval).into())
    } else {
        Ok(chip_info)
    }
}

pub fn get_line_info(
    chip_fd: FileDescriptor,
    line_offset: u32,
) -> Result<GpioLineInfo, RawOsError> {
    let mut line_info: GpioLineInfo = unsafe { std::mem::zeroed() };
    line_info.offset = line_offset;
    let retval = syscall!(
        syscall_number::IOCTL,
        chip_fd,
        ioctl_const::GET_LINE_INFO,
        &mut line_info
    );

    if retval < 0 {
        Err((-retval).into())
    } else {
        Ok(line_info)
    }
}

#[cfg(test)]
mod tests {
    use crate::platform::open;

    use super::*;

    #[test]
    fn gpio_info() {
        let chip_fd = open("/dev/gpiochip0\0", crate::platform::OpenFlags::ReadWrite).unwrap();
        let chip_info = get_chip_info(chip_fd).unwrap();
        println!("{:?}", chip_info);
        for line in 0..chip_info.lines {
            println!("{:?}", get_line_info(chip_fd, line).unwrap());
        }
    }

    fn gpio_works() {
        let chip_fd = open("/dev/gpiochip0\0", crate::platform::OpenFlags::ReadWrite).unwrap();

        let mut consumer = [0; GPIO_MAX_NAME_SIZE];
        consumer.copy_from_slice(b"");
        let mut offsets = [0; GPIO_LINES_MAX];
        offsets[0] = 0;

        let mut req = GpioLineRequest {
            offsets,
            consumer,
            num_lines: 4,
            padding: [0; 5],
            config: GpioLineConfig {
                flags: line_flag::OUTPUT,
                num_attrs: 0,
                padding: [0; 5],
                attrs: unsafe { std::mem::zeroed() },
            },
            event_buffer_size: 0,
            fd: 0,
        };
        let line_fd = get_line(chip_fd, &mut req).unwrap();
        set_values(
            line_fd,
            &GpioLineValues {
                bits: 0xf,
                mask: 0xf,
            },
        )
        .unwrap();
    }
}
