#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]
mod vga_buffer;
mod interrupts;
mod gdt;
mod memory;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
entry_point!(kernel_main);
#[no_mangle] // don't mangle the name of this function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // this function is the entry point, since the linker looks for a function
    use memory::translate_addr;
    use memory::active_level_4_table;
    use x86_64::VirtAddr;
    println!("hello World {}","!");    
    init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe {active_level_4_table(phys_mem_offset)};

    for(i,entry) in l4_table.iter().enumerate(){
        if !entry.is_unused()
        {
            println!("L4 Entry {}: {:?}",i,entry);
        }
    }


    let addresses = [0xb8000,0x201008,0x0100_0020_1a10,boot_info.physical_memory_offset,];
    for &address in &addresses{
        let virt = VirtAddr::new(address);
        let phys = unsafe {translate_addr(virt, phys_mem_offset)};
        println!("{:?} -> {:?}",virt,phys);
    }
    println!("Great SUCCESS!");
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
