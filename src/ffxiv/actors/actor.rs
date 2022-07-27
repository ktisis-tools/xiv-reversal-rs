// Dependencies

use crate::memory::MemRegion;

use std::{
	ffi::c_void,
	mem::size_of
};

// Actor

pub struct Actor {
	index: u32,
	handle: *mut c_void
}

impl Actor {
	pub fn new(index: u32, handle: *mut c_void) -> Self {
		Self { index, handle }
	}
}