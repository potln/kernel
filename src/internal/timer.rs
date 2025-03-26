use super::{mem, uart};
use crate::{read_reg, write_reg};
use mem::timer::*;

pub fn start() {
    write_reg!(TIMER1Control, 0b10100011);
}

pub fn load(value: u32) {
    clear();
    write_reg!(TIMER1Load, value);
}

pub fn read() -> u32 {
    return read_reg!(TIMER1Value, u32);
}

pub fn clear() {
    write_reg!(TIMER1IntClr, 1);
}

pub fn wait_ms(ms: u32) {
    if ms == 0 {
        return;
    }

    let ms = ms * 1_000;
    load(ms);
    start();

    while read() != 0 {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}

pub fn sync_ms(ms: u32) {
    if ms == 0 {
        return;
    }

    let ms = ms * 1_000;
    load(ms);
    start();
}

pub fn sync_wait() {
    while read() != 0 {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}

pub fn wait_s(s: u32) {
    wait_ms(s * 1_000);
}
