#![no_std] // Since we're building an operating system, we can't link to any libraries that depend on the existing operating system. This disables automatic linkage.

#![no_main] // Instead of using the main function, which starts in the C runtime library crt0, we are telling the Rust compiler not to use the normal entry point chain.

// can't use built-in test library, too unstable to port it, do we'll use the custom_test_frameworks feature to collect functions annotated with #[test_case]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

// `#[no_mangle]` macro disables name mangling, preventing compiler from turning the _start function into a randomly named function.
#[no_mangle] 
// Entry point, since the linker looks for a function named `_start` by default
pub extern "C" fn _start() -> ! {
    println!("Hello World{}","!");

    #[cfg(test)]
    test_main();

    loop{}
}

#[test_case]
fn trivial_assertion(){
    serial_print!("trivial assertion...");
    assert_eq!(1,1);
    serial_println!("[ok]");
}


// Function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

