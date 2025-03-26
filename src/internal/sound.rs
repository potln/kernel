use crate::{write_reg, read_reg};

use super::mem;
use mem::aaci::*;
pub const PL041_BASE: usize = 0x10004000;

pub const RXCR1: usize = PL041_BASE + 0x00;
pub const TXCR1: usize = PL041_BASE + 0x04;
pub const SR1: usize = PL041_BASE + 0x08;
pub const ISR1: usize = PL041_BASE + 0x0C;
pub const IE1: usize = PL041_BASE + 0x10;
pub const SL1TX: usize = PL041_BASE + 0x54;
pub const MAINCR: usize = PL041_BASE + 0x78;
pub const RESET: usize = PL041_BASE + 0x7C;
pub const DR1_0: usize = PL041_BASE + 0x90;
pub const AACIFE: u32 = 1 << 0;
pub const SL1TXEN: u32 = 1 << 4;

pub const TXEN: u32 = 1 << 0;
pub const TXSLOT3: u32 = 1 << 3;
pub const TXCOMPACT: u32 = 1 << 15;
pub const TSIZE_16BITS: u32 = 0x0 << 13;

pub const TXFF: u32 = 1 << 5;
pub const TXHE: u32 = 1 << 3;
pub const TXFE: u32 = 1 << 1;

pub fn write_reg(addr: usize, value: u32) {
    unsafe {
        core::ptr::write_volatile(addr as *mut u32, value);
    }
}

pub fn read_reg(addr: usize) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}

pub fn init() {
    write_reg(RESET, 0);
    write_reg(RESET, 1);
    write_reg(MAINCR, AACIFE | SL1TXEN);
    write_reg(TXCR1, TXEN | TXSLOT3 | TSIZE_16BITS);
}

pub fn fifo(data: u16) {
    while read_reg(SR1) & TXFF != 0 {}
    let sample = (data as u32) << 16 | (data as u32);
    write_reg(DR1_0, sample);
}

pub fn saw(sample: &mut u16, freq: u32, volume: u32) -> u16 {
    let increment = ((freq as u32) << 16) / 44100;
    *sample = sample.wrapping_add(increment as u16);
    return ((*sample as u32 * volume) >> 8) as u16;
}

pub fn read_audio_point() -> u16 {
    (read_reg(DR1_0) & 0xFFFF) as u16
}
