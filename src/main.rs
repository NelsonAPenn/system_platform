#![no_std]
#![no_main]

use system_platform::platform::write;
use system_platform::platform::STDOUT;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    system_platform::platform::exit(1)
}

fn main() {
    write(STDOUT, b"Hello, world!\n").unwrap();
}
