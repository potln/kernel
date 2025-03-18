#![no_main]
#![no_std]

mod internal;
mod panic;
mod runtime;

#[unsafe(no_mangle)]
pub fn kernel() -> ! {
    internal::uart::init();
    internal::uart::write_str("Hello");
    internal::display::init();

    loop {}
}
