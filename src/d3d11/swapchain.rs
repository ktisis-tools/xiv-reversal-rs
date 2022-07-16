// Dependencies

use libc::c_void;

use memory::MemRegion;

// SwapChain

#[repr(u16)]
enum _VTable {
	Width = 0x38,
	Height = 0x3C,
	DXGISwapChain = 0x68
}

#[derive(Debug)]
pub struct SwapChain {
	region: MemRegion
}

impl SwapChain {
	pub fn new(handle: *mut c_void) -> Self {
		let region = MemRegion::new(handle, 0x70);
		Self { region }
	}

	pub fn get<T>(&self, offset: _VTable) -> &T {
		unsafe { self.region.read(offset as isize) }
	}

	pub fn Width(&self) -> u32 {
		*self.get(_VTable::Width)
	}

	pub fn Height(&self) -> u32 {
		*self.get(_VTable::Height)
	}

	pub fn DXGISwapChain(&self) -> *mut c_void {
		*self.get(_VTable::DXGISwapChain)
	}
}