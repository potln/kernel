pub mod console;
pub mod uart;

unsafe extern "C" {
    pub safe fn syscall(call_number: u32, data: *const u8, n1: u32, n2: u32);
}
