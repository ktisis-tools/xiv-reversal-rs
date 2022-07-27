// Dependencies

use crate::{
	Vertex,
	BufferObject
};

use winapi::um::d3d11::{
	ID3D11Device,
	ID3D11DeviceContext
};

// Shapes

pub struct Shape {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u32>,
	pub buffers: BufferObject
}

impl Shape {
	// Constructors

	pub fn new(device: &ID3D11Device, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
		let buffers = BufferObject::new(device, &vertices, &indices);
		Self { vertices, indices, buffers }
	}

	pub fn rect(device: &ID3D11Device, x: f32, y: f32, w: f32, h: f32) -> Self {
		let v = vec![
			Vertex::at(x, y, 0.0),
			Vertex::at(x, y + h, 0.0),
			Vertex::at(x + w, y + h, 0.0),
			Vertex::at(x + w, y, 0.0)
		];
		let i = vec![0, 1, 2, 3, 0];
		Self::new(device, v, i)
	}

	// Draw shape

	pub fn draw(&self, devcon: &ID3D11DeviceContext) {
		self.buffers.draw(devcon)
	}
}