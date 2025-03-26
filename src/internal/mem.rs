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

pub mod aaci {
    unsafe extern "C" {
        /*
        AACIRXCR1 = .; . += 4;
        AACITXCR1 = .; . += 4;
        AACISR1 = .; . += 4;
        AACIISR1 = .; . += 4;
        AACIIE1 = .; . += 4;
        AACIRXCR2 = .; . += 4;
        AACITXCR2 = .; . += 4;
        AACISR2 = .; . += 4;
        AACIISR2 = .; . += 4;
        AACIIE2 = .; . += 4;
        AACIRXCR3 = .; . += 4;
        AACITXCR3 = .; . += 4;
        AACISR3 = .; . += 4;
        AACIISR3 = .; . += 4;
        AACIIE3 = .; . += 4;
        AACIRXCR4 = .; . += 4;
        AACITXCR4 = .; . += 4;
        AACISR4 = .; . += 4;
        AACIISR4 = .; . += 4;
        AACIIE4 = .; . += 4;
        AACISL1RX = .; . += 4;
        AACISL1TX = .; . += 4;
        AACISL2RX = .; . += 4;
        AACISL2TX = .; . += 4;
        AACISL12RX = .; . += 4;
        AACISL12TX = .; . += 4;
        AACISLFR = .; . += 4;
        AACISLISTAT = .; . += 4;
        AACISLIEN = .; . += 4;
        AACINTCLR = .; . += 4;
        AACIMAINCR = .; . += 4; 
        AACIRESET = .; . += 4;
        AACISYNC = .; . += 4;
        AACIALLINTS = .; . += 4;
        AACIMAINFR = .; . += 4;
        AACIDR1 = .; . += 4;
        AACIDR2 = .; . += 4;
        AACIDR3 = .; . += 4;
        AACIDR4 = .; . += 4; */
        pub static AACIRXCR1: *mut u32;
        pub static AACITXCR1: *mut u32;
        pub static AACISR1: *mut u32;
        pub static AACIISR1: *mut u32;
        pub static AACIIE1: *mut u32;
        pub static AACIRXCR2: *mut u32;
        pub static AACITXCR2: *mut u32;
        pub static AACISR2: *mut u32;
        pub static AACIISR2: *mut u32;
        pub static AACIIE2: *mut u32;
        pub static AACIRXCR3: *mut u32;
        pub static AACITXCR3: *mut u32;
        pub static AACISR3: *mut u32;
        pub static AACIISR3: *mut u32;
        pub static AACIIE3: *mut u32;
        pub static AACIRXCR4: *mut u32;
        pub static AACITXCR4: *mut u32;
        pub static AACISR4: *mut u32;
        pub static AACIISR4: *mut u32;
        pub static AACIIE4: *mut u32;
        pub static AACISL1RX: *mut u32;
        pub static AACISL1TX: *mut u32;
        pub static AACISL2RX: *mut u32;
        pub static AACISL2TX: *mut u32;
        pub static AACISL12RX: *mut u32;
        pub static AACISL12TX: *mut u32;
        pub static AACISLFR: *mut u32;
        pub static AACISLISTAT: *mut u32;
        pub static AACISLIEN: *mut u32;
        pub static AACINTCLR: *mut u32;
        pub static AACIMAINCR: *mut u32;
        pub static AACIRESET: *mut u32;
        pub static AACISYNC: *mut u32;
        pub static AACIALLINTS: *mut u32;
        pub static AACIMAINFR: *mut u32;
        pub static AACIDR1: *mut u32;
        pub static AACIDR2: *mut u32;
        pub static AACIDR3: *mut u32;
        pub static AACIDR4: *mut u32;
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
