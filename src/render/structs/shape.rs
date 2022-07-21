// Dependencies

use Vertex;

// Shapes

pub struct Shape2D {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u32>
}

impl Shape2D {
	pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
		Self { vertices, indices }
	}
}