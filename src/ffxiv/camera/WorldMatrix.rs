// Dependencies

use crate::{
	memory::MemRegion,
	Vec2, Vec3
};

use std::mem::transmute;

// Consts

pub type WorldMatrixFn = unsafe extern "stdcall" fn() -> *mut WorldMatrix;

// WorldMatrix

#[struct_layout::explicit(size = 0x1fc, align = 4)]
#[derive(Copy, Clone, Default, Debug)]
pub struct WorldMatrix {
	#[field(offset = 0x1b4, get, set)]
	matrix: [[f32; 4]; 4],

	#[field(offset = 0x1f4, get, set)]
	width: f32,

	#[field(offset = 0x1f8, get, set)]
	height: f32
}

impl WorldMatrix {
	pub fn find(memory: &MemRegion) -> Option<WorldMatrixFn> {
		let addr = memory.scanner().scan("E8 ?? ?? ?? ?? 48 8D 4C 24 ?? 48 89 4c 24 ?? 4C 8D 4D ?? 4C 8D 44 24 ??").delegate();
		if addr.is_null() {
			None
		} else {
			Some( unsafe { transmute::<_, WorldMatrixFn>(addr) } )
		}
	}

	pub fn transform_vec(&self, v: Vec3) -> Vec3 {
		let m = self.matrix();
		Vec3::new(
			(v.x * m[0][0]) + (v.y * m[1][0]) + (v.z * m[2][0]) + m[3][0],
			(v.x * m[0][1]) + (v.y * m[1][1]) + (v.z * m[2][1]) + m[3][1],
			(v.x * m[0][2]) + (v.y * m[1][2]) + (v.z * m[2][2]) + m[3][2]
		)
	}

	pub fn world_to_screen(&self, world: Vec3) -> Vec2 {
		let t = self.transform_vec(world);
		let vec = Vec2::new(
			0.5 * self.width() * (t.x/t.z + 1.0),
			0.5 * self.height() * (1.0 - t.y/t.z)
		);
		return vec;
	}
}

