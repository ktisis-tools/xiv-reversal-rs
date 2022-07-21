// Structs

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}
impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x, y, z }
	}
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rgba {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32
}
impl Rgba {
	pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self { r, g, b, a }
	}
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
	pos: Vec3,
	col: Rgba
}