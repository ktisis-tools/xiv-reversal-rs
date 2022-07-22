// Dependencies

use d3d11::{Device, Shaders};
use Shape;

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
		d3dcommon::{
			D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
			D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP
		}
	}
};

// Consts

const VIEWPORT_CT: usize = 1;

// RenderContext

pub struct Renderer {
	// do these need to be pointers? revisit this
	pub swapchain: *const IDXGISwapChain,
	pub device: *const ID3D11Device,
	pub shaders: Option<Shaders>,
	viewports: [D3D11_VIEWPORT; VIEWPORT_CT], // TODO: Convert to Vec?
	rtv: *mut ID3D11RenderTargetView
}

impl Renderer {
	// Constructors

	pub fn new(swapchain: *const IDXGISwapChain, device: *const ID3D11Device, shaders: Option<Shaders>) -> Self {
		let mut viewports = unsafe { [ zeroed() ] };
		let rtv = null_mut();
		Self { swapchain, device, shaders, viewports, rtv }
	}

	pub fn from(_device: Device) -> Self {
		let sc = _device.get_swapchain();
		let dev = _device.get_device();
		Self::new(sc, dev, None)
	}

	// Init values

	pub fn init(&mut self) -> &mut Self {
		// Dereference args & get context
		
		let (sc, dev) = unsafe { (&*self.swapchain, &*self.device) };
		let devcon = Device::get_context_from(dev);

		// Assign back buffer and render target pointers

		let mut back_buf: *mut c_void = null_mut();
		unsafe {
			// Get viewports
			devcon.RSGetViewports(&mut 1, self.viewports.as_mut_ptr());

			// Create render target
			let getbuf = sc.GetBuffer(0, &ID3D11Texture2D::uuidof(), &mut back_buf);
			dev.CreateRenderTargetView(back_buf as _, null_mut(), &mut self.rtv);
		}

		// TODO: Build projection matrix

		self
	}

	// Render

	pub unsafe fn render(&self) {
		let devcon = self.get_context();

		devcon.OMSetRenderTargets(1, &self.rtv, null_mut());
		devcon.RSSetViewports(VIEWPORT_CT as u32, self.viewports.as_ptr());

		if let Some(shaders) = &self.shaders {
			devcon.VSSetShader(shaders.v_shader, null_mut(), 0);
			devcon.PSSetShader(shaders.p_shader, null_mut(), 0);
			devcon.IASetInputLayout(shaders.v_input);
		}

		// Render

		devcon.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
	}

	// Get device context

	pub fn get_context(&self) -> &mut ID3D11DeviceContext {
		unsafe { Device::get_context_from(&*self.device) }
	}
}