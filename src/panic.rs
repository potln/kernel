use crate::driver;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    driver::uart::init();
    if let Some(message) = info.message().as_str() {
        driver::uart::write_str(message);
    } else {
        driver::uart::write_str("Panic occurred, but no message is available.");
    }

    loop {}
}