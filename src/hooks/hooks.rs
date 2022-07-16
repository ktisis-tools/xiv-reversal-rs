// Dependencies

use winapi::um::memoryapi::VirtualProtect;

// Hooks

pub struct Hooks {
	pub vhooks: Vec<VHook>
}

impl Hooks {
	pub fn new() -> Self {
		Self {
			vhooks: vec![]
		}
	}

	pub fn hook_vt(&mut self, handle: *mut usize, hook: *mut usize) -> VHook {
		let original = unsafe {
			VirtualProtect(handle as _, 0x1, 0x40, &mut 0x20);
			*handle as _
		};
		VHook { handle, original, hook }
	}
}

// VHook

pub struct VHook {
	pub handle: *mut usize,
	pub original: *const usize,
	pub hook: *mut usize
}

impl VHook {
	pub fn enable(&self) {
		unsafe { *self.handle = self.hook as _ };
	}

	pub fn disable(&self) {
		unsafe { *self.handle = self.original as _ };
	}
}