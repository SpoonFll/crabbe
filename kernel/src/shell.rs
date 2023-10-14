use alloc::string::String;
use core::str;
use core::sync::atomic::Ordering;

use crate::io;
use crate::{print, println};

pub fn shell() {
    print!("MR. USERMAN $> ");
    let mut command = String::new();
    while command != "exit" {
        command = gets();
        print!("\n{}", command);
    }
    println!("exiting shell now goodbye!");
}
pub fn gets() -> String {
    activate_keyboard();
    let mut ioHandler = unsafe { io::STDIO.load(Ordering::SeqCst).as_ref() }.unwrap(); //flags implemented
    while ioHandler.get_byte(ioHandler.get_position()) != b'\n' {}
    let bufRaw = ioHandler.get_buffer();
    let buffer = str::from_utf8(&bufRaw).expect("could not be decoded");
    let end = buffer.find("\n").unwrap();
    let trimmed_buffer: String = buffer.chars().take(end).collect();
    gets_exit_sequence();
    trimmed_buffer
}
fn activate_keyboard() {
    let mut ioHandler = unsafe { io::STDIO.load(Ordering::SeqCst).as_mut() }.unwrap(); //flags implemented
    ioHandler.enable_in();
    ioHandler.enable_out();
    *io::KEYBOARD_ACTIVE.try_write().unwrap() = true;
}
fn gets_exit_sequence() {
    let mut ioHandler = unsafe { io::STDIO.load(Ordering::SeqCst).as_mut() }.unwrap(); //flags implemented
    ioHandler.disable_in();
    ioHandler.disable_out();
    *io::KEYBOARD_ACTIVE.try_write().unwrap() = false;
    ioHandler.flush_buffer();
}
