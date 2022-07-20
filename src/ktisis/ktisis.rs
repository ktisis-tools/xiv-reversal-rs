// Dependencies

use process::Process;
use memory::Hooks;
use hooks;

// Ktisis

pub struct Ktisis {
	pub process: Process,
	pub hooks: Hooks
}

impl Ktisis {
	pub fn new(process: Process) -> Self {
		let hooks = Hooks::new();

		Self { process, hooks }
	}

	pub fn init(&mut self) {
		hooks::init(&mut self.hooks, &self.process);
	}
}