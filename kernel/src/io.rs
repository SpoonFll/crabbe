use alloc::boxed::Box;
use core::sync::atomic::AtomicPtr;
use lazy_static::lazy_static;
use spin::RwLock;
pub struct stdio {
    inFlag: bool,
    outFlag: bool,
    position: usize,
    stdioBuf: [u8; 1024],
}
lazy_static! {
    pub static ref STDIO: AtomicPtr<stdio> = AtomicPtr::new(Box::into_raw(Box::new(stdio {
        inFlag: false,
        outFlag: false,
        position: 0,
        stdioBuf: [0; 1024],
    })));
    pub static ref KEYBOARD_ACTIVE: RwLock<bool> = RwLock::new(true);
}
impl stdio {
    pub fn enable_in(&mut self) {
        self.inFlag = true;
    }
    pub fn disable_in(&mut self) {
        self.inFlag = false;
    }
    pub fn enable_out(&mut self) {
        self.outFlag = true;
    }
    pub fn disable_out(&mut self) {
        self.outFlag = false;
    }
    pub fn get_inFlag(&self) -> bool {
        self.inFlag
    }
    pub fn get_outFlag(&self) -> bool {
        self.outFlag
    }
    pub fn get_position(&self) -> usize {
        self.position
    }
    pub fn flush_buffer(&mut self) {
        self.position = 0;
        self.stdioBuf = [0; 1024];
    }
    pub fn append_buffer(&mut self, input_byte: u8) -> u8 {
        if self.position < 1024 {
            self.stdioBuf[self.position] = input_byte;
            self.position += 1;
            0
        } else {
            1
        }
    }
    pub fn get_byte(&self, position: usize) -> u8 {
        if position > 0 {
            self.stdioBuf[position - 1]
        } else {
            self.stdioBuf[position]
        }
    }
    pub fn del_byte(&mut self) -> u8 {
        if self.position > 0 {
            self.stdioBuf[self.position] = 0;
            self.position = self.position - 1;
            0
        } else {
            1
        }
    }
    pub fn get_buffer(&self) -> [u8; 1024] {
        self.stdioBuf
    }
}
