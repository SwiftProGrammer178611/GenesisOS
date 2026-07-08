//no std just means that we are not suing any OS, or underlying softwares. no main further 
//means don't even use the underlying built-in rust bare framework. We will be building everything OURSELVES!

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(gensisOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use gensisOS::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use gensisOS::allocator;
    use gensisOS::memory;
    use x86_64::VirtAddr;

    println!("================================");
    println!("Hello World{}", "!");
    println!("GensisOS botting up, Built with LOVE");
    println!("================================");

    gensisOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed womp womp");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(1);
    }
    println!("vec at {:p}", vec.as_slice());

    let refernce_counter = Rc::new(vec![1,2,3]);
    let cloned_refernce = refernce_counter.clone();
    println!("current ref count is {}", Rc::strong_count(&cloned_refernce));
    core::mem::drop(refernce_counter);
    println!("reference count is {} now", Rc::strong_count(&cloned_refernce));

    #[cfg(test)]
    test_main();

    println!("Great news! It has not CRASHED!");
    gensisOS::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    gensisOS::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    gensisOS::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}