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

	pub fn to_vec(&self) -> [f32; 2] {
		[self.x, self.y]
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

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Vec4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32
}
impl Vec4 {
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
		Self { x, y, z, w }
	}
}