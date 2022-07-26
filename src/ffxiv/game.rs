// Dependencies

use std::ffi::c_void;
use null_mut;

use ActorTable;

use Process;
use memory::MemRegion;

// Game Interface

pub struct GameInterface {
	actor_table: ActorTable
}

impl GameInterface {
	pub fn new() -> Self {
		Self {
			actor_table: ActorTable::new()
		}
	}

	pub fn init(&mut self, process: &Process) {
		self.actor_table.find(&process.memory);
		//camera = sigs.scan("48 8D 35 ?? ?? ?? ?? 48 8B 09");
	}
}