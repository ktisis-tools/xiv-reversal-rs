// Modules

mod present;

// Init

use memory::Hooks;
use process::Process;

pub fn init(hooks: &mut Hooks, process: &Process) {
	self::present::init(hooks, process);
}