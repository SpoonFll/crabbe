use alloc::string::String;
use core::str;
use core::sync::atomic::Ordering;

use crate::io;
use crate::{print, println};

pub fn shell() {
    print!("MR. USERMAN $> ");
    gets();
}
pub fn gets() {
    activate_keyboard();
    let mut ioHandler = unsafe { io::STDIO.load(Ordering::SeqCst).as_ref() }.unwrap(); //flags implemented
    while ioHandler.get_byte(ioHandler.get_position()) != b'\n' {}
    let bufRaw = ioHandler.get_buffer();
    let buffer = str::from_utf8(&bufRaw).expect("could not be decoded");
    let end = buffer.find("\n").unwrap();
    let trimmed_buffer: String = buffer.chars().take(end).collect();
    println!("{}", trimmed_buffer);
}
fn activate_keyboard() {
    let mut ioHandler = unsafe { io::STDIO.load(Ordering::SeqCst).as_mut() }.unwrap(); //flags implemented
    ioHandler.enable_in();
    ioHandler.enable_out();
    *io::KEYBOARD_ACTIVE.try_write().unwrap() = true;
}
