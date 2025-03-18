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