// Dependencies

use std::ffi::c_void;

// Actor

#[derive(Debug)]
pub struct Actor {
	index: u32,
	handle: *mut c_void
}

impl Actor {
	pub fn new(index: u32, handle: *mut c_void) -> Self {
		Self { index, handle }
	}
}