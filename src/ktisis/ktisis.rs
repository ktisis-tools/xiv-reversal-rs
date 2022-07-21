// Dependencies

use process::{
	Process,
	memory::Hooks
};
use render::Overlay;
use ktisis::hooks;

// Ktisis

pub struct Ktisis {
	pub process: Process,
	pub hooks: Hooks,
	pub overlay: Overlay
}

impl Ktisis {
	pub fn new(process: Process) -> Self {
		let hooks = Hooks::new();
		let overlay = Overlay::new(&process);

		Self { process, hooks, overlay }
	}

	pub fn init(&mut self) {
		self.overlay.init();
		hooks::init(&mut self.hooks, &self.process);
	}
}