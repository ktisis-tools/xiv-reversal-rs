// Vector
// TODO: Proc macros

#[derive(Copy, Clone, Debug, Default)]
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

#[derive(Copy, Clone, Debug, Default)]
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
