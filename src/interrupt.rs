use crate::internal::uart;
use core::fmt::Write;

unsafe extern "C" {
    fn boot();
}

#[unsafe(no_mangle)]
pub fn reset_handler() {
    unsafe {
        boot();
    }
}

#[unsafe(no_mangle)]
pub fn undefined_handler() {
    panic!("und");
}

#[unsafe(no_mangle)]
pub fn svc_handler(call_number: u32, data: *const u8, n1: u32, n2: u32) {
    match call_number {
        0x05 => {
            uart::call(data, n1, n2);
        }
        _ => {
            panic!("unknown svc call number");
        }
    }

    return;
}

#[unsafe(no_mangle)]
pub fn prefetch_abort_handler() {
    panic!("prefetch abort");
}

#[unsafe(no_mangle)]
pub fn data_abort_handler() {
    panic!("data abort");
}

#[unsafe(no_mangle)]
pub fn reserved_handler() {
    panic!("reserved");
}

#[unsafe(no_mangle)]
pub fn irq_handler() {
    panic!("irq");
}

#[unsafe(no_mangle)]
pub fn fiq_handler() {
    panic!("fiq");
}
