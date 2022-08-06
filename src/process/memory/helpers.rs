// Dependencies

use std::{
	mem::{size_of, zeroed},
	ffi::c_void
};

use winapi::{
	um::{
		winnt::{
			MEMORY_BASIC_INFORMATION as MEM_INFO,
			MEM_COMMIT,
			PAGE_NOACCESS
		},
		memoryapi::VirtualQuery
	}
};

// is_memory_readable
// Thanks BobTheBob - Bnuuy Central #re-and-hacking

pub fn is_memory_readable(ptr: *mut c_void) -> bool {
	if !ptr.is_null() {
		unsafe {
			let mut mem_info: MEM_INFO = zeroed();
			if VirtualQuery(ptr, &mut mem_info, size_of::<MEM_INFO>()) == 0 {
				false
			} else {
				(mem_info.State & MEM_COMMIT != 0)
				&& (mem_info.Protect & PAGE_NOACCESS == 0)
			}
		}
	} else {
		false
	}
}