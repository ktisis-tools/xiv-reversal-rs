// Dependencies

use winapi::{
	shared::minwindef::HMODULE
};

use libc::c_void;

use memory::memregion::MemRegion;

// Module

#[derive(Copy, Clone)]
pub struct Module {
	pub handle: HMODULE,
	pub memory: MemRegion
}

impl Module {
	pub fn new(handle: HMODULE, base: *mut c_void, size: usize) -> Self {
		let memory = MemRegion::new(base, size);
		Self {
			handle,
			memory
		}
	}

	// TODO: Exports
}