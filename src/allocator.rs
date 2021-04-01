use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024;

pub struct Dummy;

#[global_allocator]
static ALLOCATOR: Dummy = Dummy;

unsafe impl GlobalAlloc for Dummy {
	unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
		null_mut()
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout){
		panic!("dealloc should never be called")
	}
}