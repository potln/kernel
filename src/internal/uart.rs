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

pub fn call(data: *const u8, n1: u32, n2: u32) {
    unsafe {
        match n1 {
            0 => {
                if n2 <= u8::MAX as u32 {
                    write_byte(n2 as u8);
                } else {
                    panic!("value out of range for UART call");
                }
            }
            1 => {
                if let Some(bytes) = data.as_ref().and_then(|_| {
                    core::slice::from_raw_parts(data, n2 as usize).get(0..n2 as usize)
                }) {
                    write_bytes(bytes);
                } else {
                    panic!("invalid pointer or length for UART call");
                }
            }
            2 => {
                if let Some(bytes) = data.as_ref().and_then(|_| {
                    core::slice::from_raw_parts(data, n2 as usize).get(0..n2 as usize)
                }) {
                    if let Ok(str) = core::str::from_utf8(bytes) {
                        write_str(str);
                    } else {
                        panic!("invalid UTF-8 string passed to UART call");
                    }
                } else {
                    panic!("invalid pointer or length for UART call");
                }
            }
            3 => {
                if let Some(ptr) = (n2 as *mut u32).as_mut() {
                    let ready = ready() as u32;
                    *ptr = ready;
                } else {
                    panic!("invalid pointer passed to UART call");
                }
            }
            _ => {
                panic!("unknown UART call number");
            }
        }
    }
}
