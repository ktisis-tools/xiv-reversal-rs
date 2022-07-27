use crate::{Vec2, Vec3, Rgba};

// Vertex

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
	pos: Vec3,
	col: Rgba,
	tex: Vec2
}
impl Vertex {
	pub fn at(x: f32, y: f32, z: f32) -> Self {
		Self {
			pos: Vec3::new(x, y, z),
			col: Rgba::new(1.0, 1.0, 1.0, 1.0),
			tex: Vec2::new(0.0, 0.0)
		}
	}
}