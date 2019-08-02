#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(slice_concat_ext)]

#[macro_use]
extern crate alloc;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use rust_os::println;

pub mod lang;
pub mod scan;
pub mod parse;
pub mod interpret;
pub mod ast;
pub mod value;

entry_point!(kernel_main);


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::{memory, allocator};
    use rust_os::memory::BootInfoFrameAllocator;

    rust_os::init();

    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    lang::test_interpreter();

    #[cfg(test)]
    test_main();

    rust_os::hlt_loop()
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
