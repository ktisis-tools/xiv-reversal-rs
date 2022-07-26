// Dependencies

use std::ffi::c_void;
use null_mut;

use Process;
use process::memory::MemRegion;

// Game Memory

#[derive(Debug)]
struct GameMemory {
	actor_table: *const c_void,
	camera: *const c_void
}

// Game Interface

pub struct GameInterface {
	memory: GameMemory
}

impl GameInterface {
	pub fn new() -> Self {
		Self {
			memory: GameMemory {
				actor_table: null_mut(),
				camera: null_mut()
			}
		}
	}

	pub fn init(&mut self, process: &Process) {
		let sigs = process.memory.scanner();
		let m = &mut self.memory;
		m.actor_table = sigs.scan("48 8D 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 44 0F B6 83").asm_ptr();
		m.camera = sigs.scan("48 8D 35 ?? ?? ?? ?? 48 8B 09").asm_ptr();
	}
}