#![no_std]
#![no_main]

use core::panic::PanicInfo;

use blog_os::test_panic_handler;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	blog_os::test_panic_handler(info)
}