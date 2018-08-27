#![feature(panic_implementation)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
pub fn panic(_infor: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _stat() -> ! {
    loop {}

}
