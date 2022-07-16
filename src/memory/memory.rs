// Dependencies

use std::ffi::c_void;

use memory::SigScanner;

// MemRegion

#[derive(Copy, Clone, Debug)]
pub struct MemRegion {
	pub base: *mut c_void,
	pub size: usize
}

impl MemRegion {
	pub fn new(base: *mut c_void, size: usize) -> Self {
		Self { base, size }
	}

	pub fn scanner(&self) -> SigScanner {
		SigScanner::new(self.clone())
	}

	pub unsafe fn read<T>(&self, offset: isize) -> &T {
		&*(self.base.offset(offset) as *mut T as *const T)
	}

	pub unsafe fn read_mut<T>(&self, offset: isize) -> &mut T {
		&mut *(self.base.offset(offset) as *mut T as *mut T)
	}
}