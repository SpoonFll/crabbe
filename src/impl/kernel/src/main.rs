#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]
mod vga_buffer;
mod interrupts;
mod gdt;
use core::panic::PanicInfo;


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    println!("hello World {}","!");    
    init();
    println!("Great SUCCESS!");
    use x86_64::registers::control::Cr3;
    let(level_4_page_table, _)= Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table);
    hlt_loop();
}
fn init()
{
    gdt::init();
    interrupts::init_idt();    
    unsafe{interrupts::PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable();
}
fn hlt_loop() -> !{
    loop{
        x86_64::instructions::hlt();
    }
}
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}
