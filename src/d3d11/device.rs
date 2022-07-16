// Dependencies

use libc::c_void;

use memory::MemRegion;
use process::Process;

use d3d11::SwapChain;

// Device
// https://github.com/aers/FFXIVClientStructs/blob/main/FFXIVClientStructs/FFXIV/Client/Graphics/Kernel/Device.cs

#[derive(Debug)]
pub struct DeviceTest {
	ContextArray: *const c_void,
	RenderThread: *const c_void,
	SwapChain: *const c_void,
	D3dFeatureLevel: i32,
	DXGIFactory: *const c_void,
	DXGIOutput: *const c_void,
	D3D11Forwarder: *const c_void,
	D3D11DeviceContext: *const c_void,
	ImmediateContext: *const c_void
}

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

		let asm_ptr = unsafe { *(handle.add(0x3) as *const u32) };
		let ptr = unsafe { handle.offset(asm_ptr as isize + 0x7) } as *const usize;
		
		unsafe {
			Self::new((*ptr) as _)
		}
	}

	pub fn SwapChain(&self) {
		
	}
}