#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
extern crate alloc;
pub mod allocator;
mod gdt;
mod interrupts;
mod memory;
pub mod task;
mod vga_buffer;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use core::panic::PanicInfo;
use task::{simple_executor::SimpleExecutor, Task};

use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);
#[no_mangle] // don't mangle the name of this function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // this function is the entry point, since the linker looks for a function
    /**
     *
     * memory starts here
     *
     */
    use memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, structures::paging::Translate, VirtAddr};
    init(); //init system
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    /*
     * memory initialization finished
     *
     */

    /*
     * asynchronous function starting
     */
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();

    /*
     * make a page for memory allocation
     */
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    /*
     * write to the vga buffer
     */
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0xf021) };

    /*for (i, entry) in mapper.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }*/
    /*
     * example addresses to put on stack
     */
    let addresses = [
        0xb8000,
        0xde1000,
        0x1df000,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];
    /*
     * loops through address array to display physical addresses
     */
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }
    /*
     * heap allocation examples
     */
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    println!("Great SUCCESS!"); //end of code
    hlt_loop();
}
/**
 *
 * initializes base functions of the os
 *
 *
 */
fn init() {
    gdt::init(); //initalize global descriptor table
    interrupts::init_idt(); //initalize interrupt descriptor table
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable(); //initalize interrupts
}
fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt(); //end while loop
    }
}
/**
 * helper function for example task
 */
async fn async_number() -> u32 {
    42
}
/**
 * example async function
 * calls async_number helper function
 */
async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
/**
 * handler for allocation errors
 */
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
