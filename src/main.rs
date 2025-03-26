#![no_main]
#![no_std]

mod internal;
use core::f64::consts::PI;

use internal::*;
use kernel::driver::{self, syscall};

mod interrupt;
mod panic;
mod runtime;

#[unsafe(no_mangle)]
pub fn kernel() -> ! {
    lcd::init();
    uart::init();

    loop {
        timer::sync_ms(16);
        timer::sync_wait();
        frame::present();
    }
}
