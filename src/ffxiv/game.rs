// Dependencies

use std::{
	ffi::c_void,
	ptr::null_mut
};

use crate::{
	ActorTable,
	Camera,
	Process
};

// Game Interface

pub struct GameInterface {
	actor_table: ActorTable,
	camera: Camera
}

impl GameInterface {
	pub fn new() -> Self {
		Self {
			actor_table: ActorTable::new(),
			camera: Camera::default()
		}
	}

	pub fn init(&mut self, process: &Process) {
		self.actor_table.find(&process.memory);
	}
}