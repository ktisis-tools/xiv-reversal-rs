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

use super::actors::ModelDataPath;

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

		let mut actor = &mut self.actor_table.get_all()[0];
		println!("{:x?}", *actor as *const _ as *const usize);
		println!("{:#x?}", actor); 
		let model = actor.get_model();
		println!("{:#?}", model);
		//actor.redraw_sync();
	}
}