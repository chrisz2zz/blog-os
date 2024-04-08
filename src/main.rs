#![no_std] // 不链接rust标准库
#![no_main] // 禁用所有Rust层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::{
    println,
    task::{executor::Executor, keyboard, simple_executor::SimpleExecutor, Task},
};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

extern crate alloc;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

// 这个函数将在panic时被调用
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

#[allow(dead_code)]
static HELLO: &[u8] = b"Hello World!";

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::allocator;
    use blog_os::memory::{self, BootInfoFrameAllocator};

    // 因为链接器会寻找一个名为'_start'的函数,所以这个函数就是入口点
    // 默认命名为'_start'
    println!("Hello World{}", "!");
    // panic!("Some panic message");

    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // let heap_value = Box::new(41);
    // println!("heap_value at {:p}", heap_value);

    // let mut vec = Vec::new();
    // for i in 0..500 {
    //     vec.push(i);
    // }

    // println!("vec at {:p}", vec.as_slice());

    // let rc = Rc::new(vec![1, 2, 3]);
    // let cloned_c = rc.clone();
    // println!("current rc is {}", Rc::strong_count(&cloned_c));
    // core::mem::drop(rc);
    // println!("rc is {} now", Rc::strong_count(&cloned_c));

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    // println!("It did not crash!");
    // blog_os::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
