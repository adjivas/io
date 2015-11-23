#![feature(lang_items, start, no_std)]
#![no_std]
#![feature(core_str_ext)]

#[macro_use] extern crate io_synesthesist;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    write!("hello\n".as_ptr(), 6);
    0
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
