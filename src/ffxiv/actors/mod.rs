mod model;
pub use self::model::{ActorModel, ModelDataPath};

mod actor;
pub use self::actor::{Actor, ActorType, RenderMode};

// ActorTable

use std::{
	ffi::c_void,
	mem::size_of,
	ptr::null_mut
};

use crate::memory::MemRegion;

const ACTOR_TABLE_SIZE: u32 = 424;
const GPOSE_INDEX: u32 = 201;

pub struct ActorTable {
	pub handle: *mut c_void
}

impl ActorTable {
	pub fn new() -> Self {
		Self {
			handle: null_mut()
		}
	}

	pub fn find(&mut self, mem: &MemRegion) {
		let handle = mem.scanner().scan("48 8D 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 44 0F B6 83").asm_ptr();
		self.handle = handle;
	}

	pub fn get_all(&self) -> Vec<&mut Actor> {
		let mut result = vec![];
		if !self.handle.is_null() {
			let size = size_of::<usize>();
			for i in 0 .. ACTOR_TABLE_SIZE {
				unsafe {
					let ptr = *(self.handle.add(i as usize * size) as *mut *mut c_void);
					if !ptr.is_null() {
						result.push( &mut *(ptr as *mut Actor) );
					}
				}
			}
		}
		result
	}
}