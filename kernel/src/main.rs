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
use alloc::{boxed::Box, rc::Rc, string::String, vec, vec::Vec};
use core::panic::PanicInfo;
use task::{simple_executor::SimpleExecutor, Task};

static mut inFlag: bool = false;
static mut outFlag: bool = false;

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
    print!("[*] Initializing stack:\n");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    print!("\tPhys mem offset taken\n");
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    print!("\tMapper Initialized\n");
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    print!("\tFrame allocator Initialized\n");
    print!("[*]Stack Inialized\n");

    print!("[*] Initializing Heap: ");
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    print!("SUCCESS\n");
    /*
     * memory initialization finished
     *
     */
    print!("[+] Testing Async Functions: ");
    /*
     * asynchronous function starting
     */
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();

    /*
     * make a page for memory allocation
     */
    print!("[+] Testing Page allocation: ");
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    print!("SUCCESS empty page made\n");

    /*
     * write to the vga buffer
     */
    print!("[+] Testing raw data write to VGA buffer: ");
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(495).write_volatile(0xf021) };
    print!("SUCCESS\n");

    /*for (i, entry) in mapper.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }*/
    /*
     * example addresses to put on stack
     */
    println!("[+] Testing stack allocation 2");
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
    println!("SUCCESS");

    /*
     * heap allocation examples
     */
    println!("[+] Testing heap allocation: ");
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
    println!("SUCCESS");

    println!("Great SUCCESS! happy hacking :)"); //end of code
    println!("        \\\n         \\\n            _~^~^~_\n        \\) /  o o  \\ (/\n          '_   -   _'\n          / '-----' \\\n\n");
    //shell(); //infinite loop for now

    hlt_loop();
}
/**
 * @TODO
 * implement to place interrupt values into vec buffer by setting flag
 */
fn stdin() {}
/**
 * @TODO implement toggling STDOUT flag
 */
fn stdout() {
    print!("@TODO implement");
}
fn shell() {
    loop {
        print!("MR. USERMAN $> ");
    }
}
/**
 *
 * initializes base functions of the os
 *
 *
 */
fn init() {
    print!("[*] Initializing GDT: ");
    gdt::init(); //initalize global descriptor table
    print!("SUCESS!\n");
    print!("[*] Initializing IDT: ");
    interrupts::init_idt(); //initalize interrupt descriptor table
    print!("SUCESS!\n");
    print!("[*] Initializing INTERUPTS: ");
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable(); //initalize interrupts
    print!("SUCESS!\n");
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
    //println!("async number: {}", number);
    match number {
        42 => print!("SUCCESS code {}\n", number),
        _ => print!("FAILURE code {}\n", number),
    }
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
