// Dependencies

use crate::{
	ActorTable,
	Camera,
	Process,
	WorldMatrix,
	WorldMatrixFn
};

// Game Interface

pub struct GameInterface {
	pub actor_table: ActorTable,
	pub camera: Camera,

	pub world_matrix: Option<WorldMatrixFn>
}

impl GameInterface {
	pub fn new() -> Self {
		Self {
			actor_table: ActorTable::new(),
			camera: Camera::default(),
			
			world_matrix: None
		}
	}

	pub fn init(&mut self, process: &Process) {
		self.actor_table.find(&process.memory);
		self.world_matrix = WorldMatrix::find(&process.memory);
	}

	pub fn get_world_matrix(&self) -> WorldMatrix {
		unsafe { *self.world_matrix.unwrap()() }
	}
}