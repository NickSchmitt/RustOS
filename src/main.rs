#![no_std] // Since we're building an operating system, we can't link to any libraries that depend on the existing operating system. This disables automatic linkage.

#![no_main] // Instead of using the main function, which starts in the C runtime library crt0, we are telling the Rust compiler not to use the normal entry point chain.

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// `#[no_mangle]` macro disables name mangling, preventing compiler from turning the _start function into a randomly named function.
#[no_mangle] 
// Entry point, since the linker looks for a function named `_start` by default
pub extern "C" fn _start() -> ! {

    // println!("Hello World{}", "!");

    //  Unsafe implementation of  VGA text buffer.
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    vga_buffer::print_something();

    loop{}
}

// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}



