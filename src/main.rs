#![no_std] // Since we're building an operating system, we can't link to any libraries that depend on the existing operating system. This disables automatic linkage.

#![no_main] // Instead of using the main function, which starts in the C runtime library crt0, we are telling the Rust compiler not to use the normal entry point chain.

// can't use built-in test library, too unstable to port it, do we'll use the custom_test_frameworks feature to collect functions annotated with #[test_case]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::{hlt_loop, println};
use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::PageTable;

entry_point!(kernel_main);

// `#[no_mangle]` macro disables name mangling, preventing compiler from turning the _start function into a randomly named function.
#[no_mangle] 
// Entry point, since the linker looks for a function named `_start` by default
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello World{}","!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset)};

    for (i, entry) in l4_table.iter().enumerate(){
        if !entry.is_unused() {
            println!("L4 Entry{}: {:?}", i, entry);
            
            // get the entry's physical address
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            // print non-empty entries of the level 3 table
            for (i, entry) in l3_table.iter().enumerate(){
                if !entry.is_unused() {
                    println!(" L3 Entry {}: {:?}", i, entry);
                }
            }
        }


    }

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