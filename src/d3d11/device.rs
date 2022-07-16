// Dependencies

use libc::c_void;

use memory::MemRegion;
use process::Process;

use d3d11::SwapChain;

// Device
// https://github.com/aers/FFXIVClientStructs/blob/main/FFXIVClientStructs/FFXIV/Client/Graphics/Kernel/Device.cs

#[derive(Debug)]
pub struct Device {
	pub region: MemRegion
}

impl Device {
	pub fn new(handle: *mut c_void) -> Self {
		let region = MemRegion::new(handle, 0x220);
		Self { region }
	}

	pub fn from(process: &Process) -> Self {
		let handle = process.memory.scanner()
		.scan("48 8B 0D ?? ?? ?? ?? 48 8D 54 24 ?? F3 0F 10 44 24")
		.unwrap();

		unsafe {
			let asm_ptr = *(handle.add(0x3) as *const u32);
			let ptr = handle.offset(asm_ptr as isize + 0x7) as *const usize;
			Self::new(*ptr as _)
		}
	}

	pub fn D3DFeatureLevel(&self) -> i32 {
		unsafe { *self.region.read(0x1E8) }
	}
}