
#![no_std]
#![no_main]

use core::panic::PanicInfo;
// use k230_big::io::gpio::{Gpio, Pin, GpioMode, GpioState};
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
