use x86_64::{VirtAddr, structures::paging::{PageTable}, PhysAddr};

pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
	-> &'static mut PageTable
	{
		use x86_64::registers::control::Cr3;

		let (level_4_table_frame, _) = Cr3::read();

		let phys = level_4_table_frame.start_address();
		let virt = physical_memory_offset + phys.as_u64();
		let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

		&mut *page_table_ptr // unsafe
	}

pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr)
	-> Option<PhysAddr>
	{
		translate_addr_inner(addr, physical_memory_offset)
	}