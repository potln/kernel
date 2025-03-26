#![allow(unused)]

pub mod lcd {
    unsafe extern "C" {
        pub static _framebuffer_width: u32;
        pub static _framebuffer_height: u32;

        pub static _framebuffer_start: *mut u32;
        pub static _framebuffer_end: *mut u32;
        pub static _backbuffer_start: *mut u32;
        pub static _backbuffer_end: *mut u32;

        pub static LCDTiming0: *mut u32;
        pub static LCDTiming1: *mut u32;
        pub static LCDTiming2: *mut u32;
        pub static LCDTiming3: *mut u32;
        pub static LCDUPBASE: *mut u32;
        pub static LCDLPBASE: *mut u32;
        pub static LCDControl: *mut u16;
        pub static LCDIMSC: *mut u32;
        pub static LCDRIS: *mut u32;
        pub static LCDMIS: *mut u32;
        pub static LCDICR: *mut u32;
    }
}

pub mod uart {
    unsafe extern "C" {
        pub static UARTDR: *mut u32;
        pub static UARTRSR: *mut u32;
        pub static UARTFR: *mut u32;
        pub static UARTCR: *mut u32;
    }
}

pub mod timer {
    unsafe extern "C" {
        pub static TIMER1Load: *mut u32;
        pub static TIMER1Value: *mut u32;
        pub static TIMER1Control: *mut u32;
        pub static TIMER1IntClr: *mut u32;
        pub static TIMER1RIS: *mut u32;
        pub static TIMER1MIS: *mut u32;
        pub static TIMER1BGLoad: *mut u32;
        pub static TIMER2Load: *mut u32;
        pub static TIMER2Value: *mut u32;
        pub static TIMER2Control: *mut u32;
        pub static TIMER2IntClr: *mut u32;
        pub static TIMER2RIS: *mut u32;
        pub static TIMER2MIS: *mut u32;
        pub static TIMER2BGLoad: *mut u32;
    }
}

#[macro_export]
macro_rules! write_reg {
    ($e:expr, $value:expr) => {
        crate::internal::mem::poke(core::ptr::addr_of!($e) as *mut _, $value)
    };
}

#[macro_export]
macro_rules! read_reg {
    ($e:expr, $t:ty) => {
        crate::internal::mem::pull(core::ptr::addr_of!($e) as *mut $t)
    };
}

#[macro_export]
macro_rules! link {
    ($e:expr, $t:ty) => {
        core::ptr::addr_of!($e) as $t
    };
}

pub fn poke<T>(dst: *mut T, src: T) {
    unsafe {
        core::ptr::write_volatile(dst, src);
    }
}

pub fn pull<T>(src: *const T) -> T {
    unsafe {
        return core::ptr::read_volatile(src);
    }
}
