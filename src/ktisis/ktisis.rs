// Dependencies

use crate::{
	process::{
		Process,
		memory::Hooks
	},
	render::Overlay,
	ktisis::hooks,
	GameInterface
};

// Ktisis

pub struct Ktisis {
	pub process: Process,
	pub hooks: Hooks,
	pub game: GameInterface,
	pub overlay: Overlay
}

impl Ktisis {
	pub fn new(process: Process) -> Self {
		let hooks = Hooks::new();
		let overlay = Overlay::new(&process);
		let game = GameInterface::new();

		Self { process, hooks, overlay, game }
	}

	pub fn init(&mut self) {
		self.game.init(&self.process);
		self.overlay.init();
		hooks::init(self);
	}
}