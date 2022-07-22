// Dependencies

use libc::c_void;
use std::{
	ptr::null_mut,
	ffi::CString
};

use winapi::um::{
	d3d11::{
		ID3D11VertexShader,
		ID3D11PixelShader,
		ID3D11Device,
		ID3D11InputLayout,
		D3D11_INPUT_ELEMENT_DESC
	},
	d3dcompiler::{
		D3DCompile,
		D3DCOMPILE_ENABLE_STRICTNESS
	},
	d3dcommon::ID3DBlob,
	winnt::HRESULT,
	debugapi::OutputDebugStringA
};

// Shader

pub struct Shaders {
	pub v_shader: *mut ID3D11VertexShader,
	pub v_input: *mut ID3D11InputLayout,
	pub p_shader: *mut ID3D11PixelShader
}

impl Shaders {
	// Constructors

	pub fn new(v_shader: *mut ID3D11VertexShader, p_shader: *mut ID3D11PixelShader, v_input: *mut ID3D11InputLayout) -> Self {
		Self { v_shader, v_input, p_shader }
	}

	pub fn from_src(device: &ID3D11Device, src: &str, v_entry: Option<&str>, p_entry: Option<&str>, v_input: Vec<D3D11_INPUT_ELEMENT_DESC>) -> Self {
		let mut shaders = Self::new(null_mut(), null_mut(), null_mut());

		unsafe { 
			// Compile vertex shader
			if let Some(v_entry) = v_entry {
				if let Ok(blob) = Self::compile(src, v_entry, "vs_5_0") {
					let blob = &*blob;
					device.CreateVertexShader(blob.GetBufferPointer(), blob.GetBufferSize(), null_mut(), &mut shaders.v_shader);
					shaders.set_input(device, v_input, blob);
				}
			}

			// Compile pixel shader
			if let Some(p_entry) = p_entry {
				if let Ok(blob) = Self::compile(src, p_entry, "ps_5_0") {
					let blob = &*blob;
					device.CreatePixelShader(blob.GetBufferPointer(), blob.GetBufferSize(), null_mut(), &mut shaders.p_shader);
				}
			}
		}

		shaders
	}

	// Input layout

	pub unsafe fn set_input(&mut self, device: &ID3D11Device, inputs: Vec<D3D11_INPUT_ELEMENT_DESC>, blob: &ID3DBlob) {
		device.CreateInputLayout(
			inputs.as_ptr(),
			inputs.len() as u32,
			blob.GetBufferPointer(),
			blob.GetBufferSize(),
			&mut self.v_input
		);
	}

	// Compiler

	pub fn compile(src: &str, entry: &str, target: &str) -> Result<*const ID3DBlob, HRESULT> {
		let src = CString::new(src).unwrap();
		let entry = CString::new(entry).unwrap();
		let target = CString::new(target).unwrap();

		let mut blob: *mut ID3DBlob = null_mut();
		let mut blob_err: *mut ID3DBlob = null_mut();

		let result = unsafe {
			D3DCompile(
				src.as_ptr() as *const c_void,
				src.as_bytes().len(),
				&0,
				null_mut(),
				null_mut(),
				entry.as_ptr(),
				target.as_ptr(),
				D3DCOMPILE_ENABLE_STRICTNESS,
				0,
				&mut blob,
				&mut blob_err,
			)
		};

		if result == 0 {
			Ok(blob)
		} else {
			unsafe { OutputDebugStringA((*blob_err).GetBufferPointer() as _); }
			Err(result)
		}
	}
}