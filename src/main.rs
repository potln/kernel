#![no_main]
#![no_std]

mod internal;
use internal::*;

mod interrupt;
mod panic;
mod runtime;

#[unsafe(no_mangle)]
pub fn kernel() -> ! {
    lcd::init();
    uart::init();

    uart::write_str("Hello world!\n");

    loop {}
}
