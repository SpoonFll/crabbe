#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]
mod vga_buffer;
mod interrupts;
use core::panic::PanicInfo;


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    println!("hello World {}","!");    
    interrupts::init_idt();    

    x86_64::instructions::interrupts::int3();
println!("Great SUCCESS!");
    loop{}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}
