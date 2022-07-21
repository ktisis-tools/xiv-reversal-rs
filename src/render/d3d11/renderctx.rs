// Dependencies

use d3d11::{Device, Shaders};

use libc::c_void;
use std::{
	ptr::null_mut,
	mem::zeroed
};

use winapi::{
	Interface,
	shared::dxgi::IDXGISwapChain,
	um::{
		d3d11::{
			ID3D11Device,
			ID3D11DeviceContext,
			ID3D11RenderTargetView,
			ID3D11Texture2D,
			D3D11_VIEWPORT
		},
		d3dcommon::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST
	}
};

// Consts

const VIEWPORT_CT: usize = 1;

// RenderContext

pub struct RenderContext {
	// do these need to be pointers? revisit this
	pub swapchain: *const IDXGISwapChain,
	pub device: *const ID3D11Device,
	pub shaders: Option<Shaders>,
	viewports: [D3D11_VIEWPORT; VIEWPORT_CT]
}

impl RenderContext {
	pub fn new(swapchain: *const IDXGISwapChain, device: *const ID3D11Device, shaders: Option<Shaders>) -> Self {
		// Dereference args & get context
		
		let (sc, dev) = unsafe { (&*swapchain, &*device) };
		let devcon = Device::get_context_from(dev);

		// Get viewports
		// TODO: Aspect ratio

		let mut viewports = unsafe { [ zeroed() ] };
		unsafe {
			devcon.RSGetViewports(&mut (VIEWPORT_CT as u32), viewports.as_mut_ptr());
		}

		// Create render target

		let mut back_buf: *mut c_void = null_mut();
		let mut rtv: *mut ID3D11RenderTargetView = null_mut();

		unsafe {
			sc.GetBuffer(0, &ID3D11Texture2D::uuidof(), &mut back_buf);
			dev.CreateRenderTargetView(back_buf as _, null_mut(), &mut rtv);
		}

		// Construct render context

		Self { swapchain, device, shaders, viewports }
	}

	pub fn from(_device: Device) -> Self {
		let sc = _device.get_swapchain();
		let dev = _device.get_device();
		Self::new(sc, dev, None)
	}

	pub fn get_context(&self) -> &mut ID3D11DeviceContext {
		unsafe { Device::get_context_from(&*self.device) }
	}

	pub unsafe fn render(&self) {
		let devcon = self.get_context();

		devcon.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

		// todo: render here

		if let Some(shaders) = &self.shaders {
			devcon.VSSetShader(shaders.v_shader, null_mut(), 0);
			devcon.PSSetShader(shaders.p_shader, null_mut(), 0);
			devcon.IASetInputLayout(shaders.v_input);
		}
		devcon.RSSetViewports(VIEWPORT_CT as u32, self.viewports.as_ptr());
	}
}