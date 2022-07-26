// Dependencies

use process::Process;
use d3d11::{Device, Renderer, Shaders};

use std::include_str;
use std::ffi::CString;

use winapi::{
	um::d3d11::{
		D3D11_INPUT_ELEMENT_DESC,
		D3D11_INPUT_PER_VERTEX_DATA,
		D3D11_APPEND_ALIGNED_ELEMENT
	},
	shared::dxgiformat::{
		//DXGI_FORMAT_R32_UINT, // keep note of this.
		DXGI_FORMAT_R32G32B32A32_FLOAT,
		DXGI_FORMAT_R32G32B32_FLOAT
	}
};

// Shader code

const SHADER: &'static str = include_str!("./hlsl/shader.hlsl");

// Overlay

pub struct Overlay {
	pub renderer: Renderer
}

impl Overlay {
	// Constructor

	pub fn new(process: &Process) -> Self {
		let device = Device::from(&process);
		let renderer = Renderer::from(device);
		Self { renderer }
	}

	// Init

	pub fn init(&mut self) { // could use direct2d?
		// Init renderer

		let mut r = self.renderer.init();

		// Declared here to extend their lifetime for compilation. Not ideal, revisit this.
		let cpos = CString::new("POSITION").unwrap();
		let ccol = CString::new("COLOR").unwrap();

		// Build input layout

		let v_input = vec![
			D3D11_INPUT_ELEMENT_DESC {
				SemanticName: cpos.as_ptr(),
				SemanticIndex: 0,
				Format: DXGI_FORMAT_R32G32B32_FLOAT,
				InputSlot: 0,
				AlignedByteOffset: 0,
				InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
				InstanceDataStepRate: 0
			},
			D3D11_INPUT_ELEMENT_DESC {
				SemanticName: ccol.as_ptr(),
				SemanticIndex: 0,
				Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
				InputSlot: 0,
				AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
				InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
				InstanceDataStepRate: 0
			}
		];

		// Compile shaders

		r.shaders = Some(unsafe {
			Shaders::from_src(&*r.device, SHADER, Some("VS"), Some("PS"), v_input)
		});
	}

	// Draw

	pub fn draw(&self) {
		unsafe { self.renderer.render(); }
	}
}