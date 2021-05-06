use super::align_up;
use core::mem;


pub struct LinkedListAllocator {
	head: ListNode,
}

impl LinkedListAllocator{
	pub const fn new()->Self{
		Self {
			head: ListNode::new(0),
		}
	}

	pub unsafe fn init(&mut self, heap_start: usize, heap_size:usize){
		self.add_free_region(heap_start, heap_size);
	}

	unsafe fn add_free_region(&mut self, addr: usize, size:usize){
		todo!();
	}
}

impl LinkedListAllocator{
	// Adds the given memory region to the front of the list
	unsafe fn add_free_region(&mut self, addr: usize, size: usize){
		// ensure that the freed region is capable of holding ListNode
		assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
		assert!(size >= mem::size_of::<ListNode>());

		// create a new list node and append it at the start of the list
		let mut node = ListNode::new(size);
		node.next = self.head.next.take();
		let node_ptr = addr as *mut ListNode;
		node_ptr.write(node);
		self.head.next = Some(&mut *node_ptr)

	}
}

struct ListNode {
	size: usize,
	next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}