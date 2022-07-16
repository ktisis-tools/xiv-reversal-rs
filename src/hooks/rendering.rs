// Dependencies

use memory::{Hooks, VTable};
use process::Process;
use d3d11::Device;

use winapi::um::winnt::HRESULT;

// Globals

type PresentFunc = unsafe extern "stdcall" fn(*const usize, u32, u32) -> HRESULT;
static mut ORIGIN_PRESENT: Option<PresentFunc> = None;

// Hook functions

unsafe extern "stdcall" fn present(swap_chain: *const usize, sync_interval: u32, flags: u32) -> HRESULT {
	ORIGIN_PRESENT.unwrap()(swap_chain, sync_interval, flags)
}

// Init rendering hooks

pub fn init(hooks: &mut Hooks, process: &Process) {
	let device = Device::from(&process);

	let sc = device.SwapChain().DXGISwapChain();

	let dev = device.D3D11Forwarder();

	// this needs to be more intuitive
	unsafe {
		let PRESENT_FUNC = std::mem::transmute::<_, PresentFunc>(
			sc.inner.nth(8)
		);
		ORIGIN_PRESENT = Some(PRESENT_FUNC);
		
		let hook = hooks.hook_vt(
			sc.inner.raw_vtable().offset(8) as *mut usize,
			present as _
		);
		hook.enable();
	}
}