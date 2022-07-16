// Modules

mod rendering;

// Init

use memory::Hooks;
use process::Process;

pub fn init(hooks: &mut Hooks, process: &Process) {
	self::rendering::init(hooks, process);
}