// Dependencies

use process::Process;
use memory::Hooks;
use hooks;

use render::Overlay;

// Ktisis

pub struct Ktisis {
	pub process: Process,
	pub hooks: Hooks,
	pub overlay: Overlay
}

impl Ktisis {
	pub fn new(process: Process) -> Self {
		let hooks = Hooks::new();
		let overlay = Overlay::new();

		Self { process, hooks, overlay }
	}

	pub fn init(&mut self) {
		hooks::init(&mut self.hooks, &self.process);
		self.overlay.init();
	}
}