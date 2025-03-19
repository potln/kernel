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
pub fn undefined_handler() {}

#[unsafe(no_mangle)]
pub fn svc_handler() {}

#[unsafe(no_mangle)]
pub fn prefetch_abort_handler() {}

#[unsafe(no_mangle)]
pub fn data_abort_handler() {}

#[unsafe(no_mangle)]
pub fn reserved_handler() {}

#[unsafe(no_mangle)]
pub fn irq_handler() {}

#[unsafe(no_mangle)]
pub fn fiq_handler() {}