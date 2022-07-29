// Dependencies

use crate::{
	Process,
	d3d11::Device
};

use std::{
	mem::{transmute, zeroed},
	ptr::null_mut,
	ffi::c_void,
	time::SystemTime
};

use imgui::Context;
use imgui_dx11_renderer::Renderer;

use winapi::{
	Interface,
	um::d3d11::{
		ID3D11Device,
		ID3D11RenderTargetView,
		ID3D11Texture2D,
		D3D11_VIEWPORT
	}
};

// Overlay

pub struct Overlay {
	device: Device,

	imgui: Context,
	renderer: Renderer,

	viewport: D3D11_VIEWPORT,
	rtv: *mut ID3D11RenderTargetView,

	last_draw: SystemTime
}

impl Overlay {
	// Constructor

	pub fn new(process: &Process) -> Self {
		// DX11

		let device = Device::from(process);
		let sc = unsafe { *device.swapchain() };

		// Imgui

		let d3d11_device = unsafe { transmute::<&ID3D11Device, _>( // this is fucking stupid
			&*device.device()
		) };

		let mut imgui = Context::create();
		imgui.io_mut().display_size = [sc.width() as f32, sc.height() as f32];

		let renderer = unsafe { Renderer::new(&mut imgui, &d3d11_device).expect("Imgui renderer creation failed.") }; // why is this unsafe

		// Construct

		let viewport = unsafe { zeroed() };
		let rtv = null_mut();

		let last_draw = SystemTime::now();

		Self {
			device,

			imgui,
			renderer,
			
			viewport,
			rtv,

			last_draw
		}
	}

	// Init

	pub fn init(&mut self) {
		let dev = self.device;

		let mut back_buf: *mut c_void = null_mut();
		unsafe {
			dev.get_context().RSGetViewports(&mut 1, &mut self.viewport);

			dev.get_swapchain().GetBuffer(0, &ID3D11Texture2D::uuidof(), &mut back_buf);
			dev.get_device().CreateRenderTargetView(back_buf as _, null_mut(), &mut self.rtv);
		}
	}

	// Draw

	pub fn draw(&mut self) {
		// Setup renderer

		let devcon = self.device.get_context();
		unsafe {
			devcon.OMSetRenderTargets(1, &self.rtv, null_mut());
			devcon.RSSetViewports(1, &mut self.viewport);
		}

		// Calculate delta time

		let now = SystemTime::now();
		let delta_time = now.duration_since(self.last_draw).unwrap().as_nanos() as f32 / 1_000_000_000.0;

		self.imgui.io_mut().delta_time = delta_time;
		
		// Draw frame

		let ui = self.imgui.frame();

		{
			let draw_list = ui.get_foreground_draw_list();

			// TODO: Draw something
		}

		self.renderer.render( ui.render() ).expect("Imgui frame failed.");

		self.last_draw = now;
	}
}