// Dependencies

use crate::{
	Actor,
	memory::{
		MemRegion,
		is_memory_readable
	}
};

use std::ptr::null_mut;

// Consts

const ACTOR_TABLE_SIZE: u32 = 424;
const GPOSE_INDEX: u32 = 201;

pub struct ActorTable {
	pub handle: *mut *mut Actor
}

impl ActorTable {
	pub fn new() -> Self {
		Self {
			handle: null_mut()
		}
	}

	pub fn find(&mut self, mem: &MemRegion) {
		let handle = mem.scanner().scan("48 8D 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 44 0F B6 83").asm_ptr();
		self.handle = handle as *mut _;
	}

	pub fn get(&self, i: usize) -> Option<&mut Actor> {
		unsafe {
			let ptr = *(self.handle.add(i) as *mut *mut Actor);
			if ptr.is_null() {
				None
			} else {
				Some(&mut *ptr)
			}
		}
	}

	pub fn get_all(&self) -> Vec<&mut Actor> {
		let mut result = vec![];
		if !self.handle.is_null() {
			for i in 0 .. ACTOR_TABLE_SIZE {
				unsafe {
					let ptr = *(self.handle.add(i as usize) as *mut *mut Actor);
					if is_memory_readable(ptr as _) {
						result.push(&mut *ptr);
					}
				}
			}
		}
		result
	}
}