// Dependencies

use crate::{
	memory::{MemRegion, VTable},
	process::Process
};

use libc::c_void;
use std::ptr::null_mut;

use winapi::{
	um::d3d11::{
		ID3D11Device,
		ID3D11DeviceContext
	},
	shared::dxgi::IDXGISwapChain
};

// Device
// https://github.com/aers/FFXIVClientStructs/blob/main/FFXIVClientStructs/FFXIV/Client/Graphics/Kernel/Device.cs

#[repr(u16)]
enum _Offset {
	IDXGISwapChain = 0x58,
	SwapChainVTable = 0x68,
	D3D11Forwarder = 0x200,
	D3D11DeviceContext = 0x208
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
		let res = process.memory.scanner()
		.scan("48 8B 0D ?? ?? ?? ?? 48 8D 54 24 ?? F3 0F 10 44 24")
		.asm_ptr();

		Self::new(unsafe { *(res as *mut *mut _) })
	}

	fn get<T>(&self, offset: _Offset) -> &T {
		unsafe { self.region.read(offset as isize) }
	}

	pub fn get_swapchain(&self) -> &IDXGISwapChain {
		let sc_handle: *mut c_void = *self.get(_Offset::IDXGISwapChain);
		let vt_handle: *const IDXGISwapChain = unsafe { *( sc_handle.offset(_Offset::SwapChainVTable as isize) as *const _ ) };
		unsafe { &*vt_handle }
	}
	pub fn get_swapchain_vt(&self) -> VTable {
		VTable::new( self.get_swapchain() as *const _ as _ )
	}

	pub fn get_device(&self) -> &ID3D11Device {
		*self.get(_Offset::D3D11Forwarder)
	}

	pub fn get_context(&self) -> &ID3D11DeviceContext {
		*self.get(_Offset::D3D11DeviceContext)
	}

	pub fn get_context_from(device: &ID3D11Device) -> &mut ID3D11DeviceContext {
		let mut devcon: *mut ID3D11DeviceContext = null_mut();
		unsafe {
			device.GetImmediateContext(&mut devcon);
			&mut *devcon
		}
	}
}