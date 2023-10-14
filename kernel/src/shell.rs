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
    println!(
        "{}\n",
        str::from_utf8(&ioHandler.get_buffer()).expect("could not be decoded")
    );
}
fn activate_keyboard() {
    let mut ioHandler = unsafe { io::STDIO.load(Ordering::SeqCst).as_mut() }.unwrap(); //flags implemented
    ioHandler.enable_in();
    ioHandler.enable_out();
    *io::KEYBOARD_ACTIVE.try_write().unwrap() = true;
}
