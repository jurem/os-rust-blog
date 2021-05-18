// https://github.com/phil-opp/blog_os

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


extern crate alloc;

mod vga_buffer;

use core::panic::PanicInfo;


/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
	blog_os::hlt_loop();
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}


// ********************** tasks

async fn async_number() -> u32 {
	42
}

async fn example_task() {
	let number = async_number().await;
	println!("async number: {}", number);
}


// ***************************** main

use bootloader::{BootInfo, entry_point};
use blog_os::task::{Task, executor::Executor};
use blog_os::task::keyboard;

entry_point!(kernel_main);

fn kernel_main(bootinfo: &'static BootInfo) -> ! {
	println!("Hello World{}", "!", );
	blog_os::init();
    // **********************

    use x86_64::VirtAddr;
    use blog_os::allocator;
	use blog_os::memory::{self, BootInfoFrameAllocator};

    let phys_mem_offset = VirtAddr::new(bootinfo.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
    	BootInfoFrameAllocator::init(&bootinfo.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
    	.expect("heap initialization failed");

    // *************************

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    // **********************
    #[cfg(test)]
    test_main();
	println!("This is the end my friend, the end.");
	blog_os::hlt_loop();
}




#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
