use super::mem;
use crate::{link, write_reg};
use mem::lcd::*;

pub fn timing(up: u32, m1: u32, m2: u32, low: u32) -> u32 {
    (up << 24) | (m1 << 16) | (m2 << 10) | low
}

pub fn control(
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
    let enable_bit = if enable { 1 } else { 0 };
    let bpp_bits = ((bpp as u32) & 0b111) << 1;
    let bw_bit = if bw { 1 << 4 } else { 0 };
    let tft_bit = if tft { 1 << 5 } else { 0 };
    let mono8_bit = if mono8 { 1 << 6 } else { 0 };
    let dual_bit = if dual { 1 << 7 } else { 0 };
    let bgr_bit = if bgr { 1 << 8 } else { 0 };
    let bebo_bit = if bebo { 1 << 9 } else { 0 };
    let bepo_bit = if bepo { 1 << 10 } else { 0 };
    let power_bit = if power { 1 << 11 } else { 0 };
    let vcomp_bits = ((vcomp as u32) & 0b11) << 12;
    let watermark_bit = if watermark { 1 << 16 } else { 0 };

    return enable_bit
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
        | watermark_bit;
}

pub fn init() {
    write_reg!(
        LCDTiming0,
        timing(160, 24, 136, link!(_framebuffer_width, u32) - 1)
    );
    write_reg!(
        LCDTiming1,
        timing(29, 3, 6, link!(_framebuffer_height, u32) - 1)
    );
    write_reg!(LCDUPBASE, link!(_framebuffer_end, u32));
    write_reg!(
        LCDControl,
        control(
            true, 0b101, false, true, false, false, false, false, false, true, 0b01, false,
        )
    );
}
