// Vector
// TODO: Proc macros

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32
}
impl Vec2 {
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}
}

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

// Color

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