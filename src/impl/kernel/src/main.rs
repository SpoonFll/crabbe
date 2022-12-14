#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]
mod gdt;
mod interrupts;
mod memory;
mod vga_buffer;
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
entry_point!(kernel_main);
#[no_mangle] // don't mangle the name of this function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // this function is the entry point, since the linker looks for a function
    use x86_64::{structures::paging::Page,structures::paging::Translate, VirtAddr};
    use memory::BootInfoFrameAllocator;
    println!("hello World {}", "!");
    init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {BootInfoFrameAllocator::init(&boot_info.memory_map)};

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe{page_ptr.offset(400).write_volatile(0xf021)};

    /*for (i, entry) in mapper.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }*/

    let addresses = [
        0xb8000,
        0xde1000,
        0x1df000,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }
    println!("Great SUCCESS!");
    println!("{}", boot_info.physical_memory_offset);
    hlt_loop();
}
fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
