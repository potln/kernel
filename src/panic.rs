use crate::internal;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    internal::uart::init();
    if let Some(message) = info.message().as_str() {
        internal::uart::write_str(message);
    } else {
        internal::uart::write_str("Panic occurred, but no message is available.");
    }

    loop {}
}
