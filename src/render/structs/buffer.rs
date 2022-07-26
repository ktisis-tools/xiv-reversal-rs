// Dependencies

use Vertex;

use std::{
	ptr::null_mut,
	mem::size_of
};

use winapi::{
	shared::dxgiformat::DXGI_FORMAT_R32_UINT,
	um::{
		//d3dcommon::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
		d3d11::{
			ID3D11Buffer,
			ID3D11Device,
			ID3D11DeviceContext,
			D3D11_BUFFER_DESC,
			D3D11_SUBRESOURCE_DATA,
			D3D11_BIND_VERTEX_BUFFER,
			D3D11_BIND_INDEX_BUFFER
		}
	}
};

// Consts

const VERT_SIZE: u32 = size_of::<Vertex>() as _;
const U32_SIZE: u32 = size_of::<u32>() as _;

// BufferObject

pub struct BufferObject {
	pub v_len: u32,
	pub i_len: u32,
	pub v_buffer: *mut ID3D11Buffer,
	pub i_buffer: *mut ID3D11Buffer
}

impl BufferObject {
	pub fn new(device: &ID3D11Device, vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Self {
		let v_len = vertices.len() as _;
		let i_len = indices.len() as _;

		// Vertex buffer

		let v_desc = D3D11_BUFFER_DESC {
			ByteWidth: VERT_SIZE * v_len as u32,
			BindFlags: D3D11_BIND_VERTEX_BUFFER,
			Usage: 0,
			CPUAccessFlags: 0,
			MiscFlags: 0,
			StructureByteStride: VERT_SIZE
		};
		let v_data = D3D11_SUBRESOURCE_DATA {
			pSysMem: vertices.as_ptr() as _,
			SysMemPitch: 0,
			SysMemSlicePitch: 0
		};

		// Index buffer

		let i_desc = D3D11_BUFFER_DESC {
			ByteWidth: U32_SIZE * i_len,
			BindFlags: D3D11_BIND_INDEX_BUFFER,
			Usage: 0,
			CPUAccessFlags: 0,
			MiscFlags: 0,
			StructureByteStride: U32_SIZE
		};
		let i_data = D3D11_SUBRESOURCE_DATA {
			pSysMem: indices.as_ptr() as _,
			SysMemPitch: 0,
			SysMemSlicePitch: 0
		};

		// Build buffers

		let mut v_buffer: *mut ID3D11Buffer = null_mut();
		let mut i_buffer: *mut ID3D11Buffer = null_mut();
		unsafe {
			device.CreateBuffer(&v_desc, &v_data, &mut v_buffer);
			device.CreateBuffer(&i_desc, &i_data, &mut i_buffer);
		}
		Self { v_len, i_len, v_buffer, i_buffer }
	}

	pub fn draw(&self, devcon: &ID3D11DeviceContext) {
		unsafe {
			devcon.IASetVertexBuffers(0, 1, &self.v_buffer, &VERT_SIZE, &0);
			devcon.IASetIndexBuffer(self.i_buffer, DXGI_FORMAT_R32_UINT, 0);
			devcon.DrawIndexed(self.i_len, 0, 0);
		}
	}
}