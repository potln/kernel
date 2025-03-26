use crate::{link, read_reg, write_reg};

use super::mem;
use mem::lcd::*;

pub enum Buffer {
    Front,
    Back,
}

pub fn side() -> Buffer {
    let frame_addr: u32 = link!(_framebuffer_end, u32);

    return if addr() == frame_addr {
        Buffer::Back
    } else {
        Buffer::Front
    };
}

pub fn width() -> usize {
    return link!(_framebuffer_width, usize);
}

pub fn height() -> usize {
    return link!(_framebuffer_height, usize);
}

pub fn write_pixel(x: usize, y: usize, color: u32) {
    if (x >= width()) || (y >= height()) {
        return;
    }

    let framebuffer_width = width();
    let framebuffer_start = buffer();

    return unsafe {
        framebuffer_start
            .add(y * framebuffer_width + x)
            .write_volatile(color);
    };
}

pub fn read_pixel(x: usize, y: usize) -> u32 {
    let framebuffer_width = width();
    let framebuffer_start = buffer();

    return unsafe {
        framebuffer_start
            .add(y * framebuffer_width + x)
            .read_volatile()
    };
}

pub fn fill_rect(x: usize, y: usize, width: usize, height: usize, color: u32) {
    let framebuffer_width = self::width();
    let framebuffer_start = buffer();

    for dy in 0..height {
        for dx in 0..width {
            unsafe {
                framebuffer_start
                    .add((y + dy) * framebuffer_width + x + dx)
                    .write_volatile(color);
            }
        }
    }
}

pub fn box_rect(x: usize, y: usize, width: usize, height: usize, border: usize, color: u32) {
    fill_rect(x, y, width, border, color);
    fill_rect(x, y, border, height, color);
    fill_rect(x + width - border, y, border, height, color);
    fill_rect(x, y + height - border, width, border, color);
}

pub fn fill(color: u32) {
    let framebuffer_width = self::width();
    let framebuffer_height = self::height();
    let framebuffer_start = buffer();

    for dy in 0..framebuffer_height {
        for dx in 0..framebuffer_width {
            unsafe {
                framebuffer_start
                    .add(dy * framebuffer_width + dx)
                    .write_volatile(color);
            }
        }
    }
}

pub fn addr() -> u32 {
    return read_reg!(LCDUPBASE, u32);
}

pub fn buffer() -> *mut u32 {
    return match side() {
        Buffer::Front => link!(_framebuffer_end, *mut u32),
        Buffer::Back => link!(_backbuffer_end, *mut u32),
    };
}

pub fn present() {
    write_reg!(LCDUPBASE, buffer());
}
