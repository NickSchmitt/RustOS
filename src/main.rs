#![no_std] // Since we're building an operating system, we can't link to any libraries that depend on the existing operating system. This disables automatic linkage.

#![no_main] // Instead of using the main function, which starts in the C runtime library crt0, we are telling the Rust compiler not to use the normal entry point chain.

// can't use built-in test library, too unstable to port it, do we'll use the custom_test_frameworks feature to collect functions annotated with #[test_case]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::{hlt_loop, println};

// `#[no_mangle]` macro disables name mangling, preventing compiler from turning the _start function into a randomly named function.
#[no_mangle] 
// Entry point, since the linker looks for a function named `_start` by default
pub extern "C" fn _start() -> ! {
    println!("Hello World{}","!");

    blog_os::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    
    blog_os::hlt_loop();
}

// Function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}