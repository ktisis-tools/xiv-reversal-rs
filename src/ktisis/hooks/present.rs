// Dependencies

use Ktisis;

use process::{
	Process,
	memory::Hooks
};
use render::{
	Overlay,
	d3d11::Device
};

use std::mem::transmute;

use winapi::{
	um::winnt::HRESULT,
	shared::dxgi::IDXGISwapChain
};

// Consts

type Present = unsafe extern "stdcall" fn(*mut IDXGISwapChain, u32, u32) -> HRESULT;
static mut ORIGIN: Option<Present> = None;

static mut KTISIS: Option<*mut Ktisis> = None;

// Present

unsafe extern "stdcall" fn present(
	swap_chain: *mut IDXGISwapChain,
	sync_interval: u32,
	flags: u32
) -> HRESULT {
	if let Some(ktisis) = KTISIS {
		let ktisis = &*ktisis;
		ktisis.overlay.draw();
	}
	
	if let Some(call) = &ORIGIN {
		call(swap_chain, sync_interval, flags)
	} else { 0 }
}

// Init hook

pub fn init(ktisis: &mut Ktisis) {
	let device = Device::from(&ktisis.process);
	let dev = device.get_device();

	let sc_vt = device.get_swapchain_vt();

	// this needs to be more intuitive
	unsafe {
		KTISIS = Some(ktisis);

		let present_fn = transmute::<_, Present>(
			sc_vt.nth(8)
		);
		ORIGIN = Some(present_fn);
		
		let hook = ktisis.hooks.hook_vt(
			sc_vt.raw_vtable().offset(8) as *mut usize,
			present as _
		);

		println!("Hook created. Replacing VTable entry at {:x?} with {:x?} (originally {:x?})", hook.handle, hook.hook, hook.original);

		hook.enable();
	}
}