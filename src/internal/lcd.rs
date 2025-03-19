use crate::write_reg;
use super::mem;
use mem::lcd::*;

/// Calculate the value for the LCDTiming0 register
fn calculate_lcd_timing0(hbp: u32, hfp: u32, hsw: u32, ppl: u32) -> u32 {
    (hbp << 24) | (hfp << 16) | (hsw << 8) | ppl
}

/// Calculate the value for the LCDTiming1 register
fn calculate_lcd_timing1(vbp: u32, vfp: u32, vsw: u32, lpp: u32) -> u32 {
    (vbp << 24) | (vfp << 16) | (vsw << 10) | lpp
}

/// Calculate the value for the LCDUPBASE register
fn calculate_lcd_upbase(framebuffer_addr: u32) -> u32 {
    framebuffer_addr
}

/// Calculate the value for the LCDControl register
fn calculate_lcd_control(
    enable: bool,
    bpp: u8,
    bw: bool,
    tft: bool,
    mono8: bool,
    dual: bool,
    bgr: bool,
    bebo: bool,
    bepo: bool,
    power: bool,
    vcomp: u8,
    watermark: bool,
) -> u32 {
    let enable_bit = if enable { 1 } else { 0 }; // Bit 0
    let bpp_bits = ((bpp as u32) & 0b111) << 1; // Bits 1-3
    let bw_bit = if bw { 1 << 4 } else { 0 };   // Bit 4
    let tft_bit = if tft { 1 << 5 } else { 0 }; // Bit 5
    let mono8_bit = if mono8 { 1 << 6 } else { 0 }; // Bit 6
    let dual_bit = if dual { 1 << 7 } else { 0 };   // Bit 7
    let bgr_bit = if bgr { 1 << 8 } else { 0 };     // Bit 8
    let bebo_bit = if bebo { 1 << 9 } else { 0 };   // Bit 9
    let bepo_bit = if bepo { 1 << 10 } else { 0 };  // Bit 10
    let power_bit = if power { 1 << 11 } else { 0 }; // Bit 11
    let vcomp_bits = ((vcomp as u32) & 0b11) << 12; // Bits 12-13
    let watermark_bit = if watermark { 1 << 16 } else { 0 }; // Bit 16

    enable_bit
        | bpp_bits
        | bw_bit
        | tft_bit
        | mono8_bit
        | dual_bit
        | bgr_bit
        | bebo_bit
        | bepo_bit
        | power_bit
        | vcomp_bits
        | watermark_bit
}

pub fn init() {
    let timing0 = calculate_lcd_timing0(255, 255, 255, 5);
    let timing1 = calculate_lcd_timing1(9, 3, 4, 767);
    unsafe {
        let upbase = calculate_lcd_upbase(core::ptr::addr_of!(_framebuffer_end) as u32);
        let control = calculate_lcd_control(
            true,    // enable
            0b101,   // bpp (16 BPP)
            false,   // bw
            true,    // tft
            false,   // mono8
            false,   // dual
            false,   // bgr
            false,   // bebo
            false,   // bepo
            true,    // power
            0b01,    // vcomp
            false,   // watermark
        );

        // Debug messages
        crate::internal::uart::write_str("LCDUpbase: ");
        let mut timing0_str = [0u8; 8];
        for i in 0..8 {
            let nibble = (upbase >> (28 - i * 4)) & 0xF;
            timing0_str[i] = if nibble < 10 {
            b'0' + nibble as u8
            } else {
            b'A' + (nibble as u8 - 10)
            };
        }
        crate::internal::uart::write_bytes(&timing0_str);
        crate::internal::uart::write_str("\n");

        crate::internal::uart::write_str("LCDUPBASE: ");
        crate::internal::uart::write_byte((upbase >> 24) as u8);
        crate::internal::uart::write_byte((upbase >> 16) as u8);
        crate::internal::uart::write_byte((upbase >> 8) as u8);
        crate::internal::uart::write_byte(upbase as u8);
        crate::internal::uart::write_str("\n");

        write_reg!(LCDTiming0, timing0);
        write_reg!(LCDTiming1, timing1);
        write_reg!(LCDUPBASE, upbase);
        write_reg!(LCDControl, control); // Use the calculated control value
    }
}