#![no_main]
#![no_std]

mod driver;
mod runtime;
mod panic;

#[unsafe(no_mangle)]
pub fn kernel() -> ! {
    driver::uart::init();
    driver::uart::write_str("Hello");
    
    loop {
    }
}