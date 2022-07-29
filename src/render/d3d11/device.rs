// Dependnecies

use crate::{
	Process,
	memory::VTable
};

use winapi::{
	um::d3d11::{
		ID3D11Device,
		ID3D11DeviceContext
	},
	shared::dxgi::IDXGISwapChain
};

// Device

#[struct_layout::explicit(size = 0x220, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct Device {
	#[field(offset = 0x58, get, set)]
	pub swapchain: *mut SwapChainWrapper,
	#[field(offset = 0x200, get, set)]
	pub device: *mut ID3D11Device,
	#[field(offset = 0x208, get, set)]
	pub context: *mut ID3D11DeviceContext
}

impl Device {
	pub fn from(process: &Process) -> Self {
		let res = process.memory.scanner()
		.scan("48 8B 0D ?? ?? ?? ?? 48 8D 54 24 ?? F3 0F 10 44 24")
		.asm_ptr();

		unsafe { **(res as *mut *mut Self) }
	}

	pub fn get_swapchain(&self) -> &IDXGISwapChain {
		let wrapper = self.swapchain();
		let inner = unsafe { (*wrapper).inner() };
		unsafe { &*inner }
	}

	pub fn get_swapchain_vt(&self) -> VTable {
		VTable::new( self.get_swapchain() as *const _ as _ )
	}

	pub fn get_device(&self) -> &ID3D11Device { unsafe { &*self.device() } }
	pub fn get_context(&self) -> &ID3D11DeviceContext { unsafe { &*self.context() } }
}

// SwapChainWrapper

#[struct_layout::explicit(size = 0x70, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct SwapChainWrapper {
	#[field(offset = 0x38, get, set)]
	pub width: u32,
	#[field(offset = 0x3c, get, set)]
	pub height: u32,
	#[field(offset = 0x68, get, set)]
	pub inner: *const IDXGISwapChain
}