use super::mem;

const UART_BASE: u32 = 0x10009000;
const UART_CR: u32 = UART_BASE + 0x0C;
const UART_FR: u32 = UART_BASE + 0x18;

pub fn init() {
    mem::poke(UART_CR as *mut u32, 0x3);
}

fn ready() -> bool {
    let fr = mem::pull(UART_FR as *const u32);
    return (fr & (1 << 5)) == 0;
}

pub fn write_byte(byte: u8) {
    while !ready() {}
    mem::poke(UART_BASE as *mut u8, byte);
}

pub fn write_bytes(bytes: &[u8]) {
    for &byte in bytes {
        write_byte(byte);
    }
}

pub fn write_str(str: &'static str) {
    write_bytes(str.as_bytes());
}
