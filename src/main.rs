#![no_main]
#![no_std]

mod internal;

use internal::{frame::fill_rect, *};

mod interrupt;
mod panic;
mod runtime;

#[unsafe(no_mangle)]
pub fn kernel() -> ! {
    lcd::init();
    uart::init();

    sound::init();

    let mut sample: u16 = 0;
    let mut time: u16 = 0;
    let mut last = 0;

    let w = 300;
    let h = 150;

    loop {
        if time as usize % w == 0 {
            frame::present();
            frame::fill_rect(0, 0, w, h, 0xff);
        }

        let audio_sample = sound::saw(&mut sample, 300, 25);
        sound::fifo(audio_sample);

        let x = time as usize % w;
        let y = (audio_sample as usize * h) / u16::MAX as usize;

        if last > y + 1 {
            fill_rect(x, y, 1, last - y, 0xffff);
        }

        frame::write_pixel(x, y, 0xFFFF);
        last = y;

        time = time.wrapping_add(1);
    }
}
