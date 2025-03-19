use super::mem;
use crate::{read_reg, write_reg};
use mem::uart::*;

pub fn init() {
    write_reg!(UARTCR, 0x3);
}

pub fn ready() -> bool {
    let fr = read_reg!(UARTFR, u32);
    return (fr & (1 << 5)) == 0;
}

pub fn write_byte(byte: u8) {
    while !ready() {}
    write_reg!(UARTDR, byte);
}

pub fn write_bytes(bytes: &[u8]) {
    for &byte in bytes {
        write_byte(byte);
    }
}

pub fn write_str(str: &'static str) {
    write_bytes(str.as_bytes());
}
