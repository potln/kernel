use super::syscall;

pub fn write_byte(byte: u8) {
    syscall(0x05, core::ptr::null(), 0, byte as u32);
}

pub fn write_bytes(bytes: &[u8]) {
    syscall(0x05, bytes.as_ptr(), 1, 0);
}

pub fn write_str(str: &'static str) {
    syscall(0x05, str.as_bytes().as_ptr(), 2, 0);
}
