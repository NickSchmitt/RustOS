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

	// Looks for free region and removes it from the list to use it for allocation
	// Returns tuple of the list node and the allocation start address
	fn find_region(&mut self, size: usize, align: usize)
		-> Option<(&'static mut ListNode, usize)>
	{
		// reference to current node
		let mut current = &mut self.head;
		// iterate over linked list, searching for sufficiently large free region
		while let Some(ref mut region) = current.next {
			if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align){
				// If region is suitable, remove free region to prepare for allocation
				let next = region.next.take();
				let ret = Some((current.next.take().unwrap(), alloc_start));
				current.next = next;
				return ret;
			} else {
				// Region unsuitable, continue iterative search
				current = current.next.as_mut().unwrap();
			}

		}
		// No suitable region
		None
	}

	// Function that tests whether a memory region is suitable for allocation
	// Returns start address on success
	fn alloc_from_region(region: &ListNode, size: usize, align: usize)
	-> Result<usize, ()>
	{
		let alloc_start = align_up(region.start_addr(), align);
		let alloc_end = alloc_start.checked_add(size).ok_or(())?;

		if alloc_end > region.end_addr(){
			// insufficient size
			return Err(());
		}

		let excess_size = region.end_addr() - alloc_end;
		if excess_size > 0 && excess_size < mem::size_of::<ListNode>(){
			return Err(());
		}

		Ok(alloc_start)
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