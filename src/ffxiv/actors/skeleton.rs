// Dependencies

use crate::Vec4;

use std::ffi::CStr;
use libc::{c_char, c_void};

// Consts

const POSE_CT: usize = 4;

// TODO: Organise all of this into individual files.

/*
	Memory Arrays
	TODO: MACRO THIS
*/

// SkeletonArray

#[struct_layout::explicit(size = 0x70, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct SkeletonArray {
	#[field(offset = 0x50, get, set)]
	pub count: i16,
	#[field(offset = 0x68, get, set)]
	pub handle: *mut Skeleton
}

impl SkeletonArray {
	pub fn get_vec(&self) -> Vec<&mut Skeleton> {
		let ct = self.count() as usize;
		let handle = self.handle();

		let mut vec = Vec::with_capacity(ct);
		for i in 0 .. ct {
			vec.push( unsafe { &mut *(handle.add(i)) } );
		}
		vec
	}
}

// TODO: ParentArray

// Skeleton

#[struct_layout::explicit(size = 0x1c0, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct Skeleton {
	#[field(offset = 0x12c, get, set)]
	pub bone_node_index: i16,
	#[field(offset = 0x12e, get, set)]
	pub bone_parent_index: i16,

	#[field(offset = 0x140, get, set)]
	pub poses: [*mut HkaPose; POSE_CT]
}

impl Skeleton {
	pub fn get_poses(&self) -> Vec<&mut HkaPose> {
		let mut vec = Vec::with_capacity(POSE_CT);
		for i in 0 .. POSE_CT {
			vec.push( self.get_pose(i) );
		}
		vec
	}

	pub fn get_pose(&self, i: usize) -> &mut HkaPose {
		unsafe { &mut *self.poses()[i] }
	}
}

// HkaPose

#[struct_layout::explicit(size = 0x20, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct HkaPose {
	#[field(offset = 0x0, get, set)]
	pub skeleton: *mut HkaSkeleton,
	#[field(offset = 0x10, get, set)]
	pub transforms: TransformArray
}

impl HkaPose {
	pub fn get_skeleton(&self) -> &mut HkaSkeleton {
		unsafe { &mut *self.skeleton() }
	}
}

// TransformArray

#[struct_layout::explicit(size = 0x10, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct TransformArray {
	#[field(offset = 0x8, get, set)]
	pub handle: *mut Transform,
	#[field(offset = 0x0, get, set)]
	pub count: u32
}

impl TransformArray {
	pub fn get_vec(&self) -> Vec<&mut Transform> {
		let ct = self.count() as usize;
		let mut vec = Vec::with_capacity(ct);
		for i in 0 .. ct {
			vec.push( unsafe { &mut *(self.handle().add(i)) } );
		}
		vec
	}
}

// Transform

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Transform {
	translate: Vec4,
	rotate: Vec4,
	scale: Vec4
}

// HkaSkeleton

#[struct_layout::explicit(size = 0x38, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct HkaSkeleton {
	#[field(offset = 0x10, get, set)]
	pub name: *mut c_char,
	#[field(offset = 0x18, get, set)]
	pub parent_index: ParentArray,
	#[field(offset = 0x28, get, set)]
	pub bones: BoneArray
}

impl HkaSkeleton {
	pub fn get_name(&self) -> String {
		unsafe { CStr::from_ptr(self.name()).to_str().unwrap().to_string() }
	}
}

// BoneArray

#[struct_layout::explicit(size = 0x10, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct BoneArray {
	#[field(offset = 0x0, get, set)]
	pub handle: *mut HkaBone,
	#[field(offset = 0x8, get, set)]
	pub count: u32
}

impl BoneArray {
	pub fn get_vec(&self) -> Vec<&mut HkaBone> {
		let ct = self.count() as usize;
		let mut vec = Vec::with_capacity(ct);
		for i in 0 .. ct {
			vec.push( unsafe { &mut *(self.handle().add(i)) } );
		}
		vec
	}
}

// HkaBone

#[struct_layout::explicit(size = 0x10, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct HkaBone {
	#[field(offset = 0x0, get, set)]
	name: *mut c_char
}

impl HkaBone {
	pub fn get_name(&self) -> String {
		unsafe { CStr::from_ptr(self.name()).to_str().unwrap().to_string() }
	}
}

// ParentArray

#[struct_layout::explicit(size = 0x10, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct ParentArray {
	#[field(offset = 0x0, get, set)]
	pub handle: *mut i16,
	#[field(offset = 0x8, get, set)]
	pub count: u32
}

impl ParentArray {
	pub fn get_vec(&self) -> Vec<&mut i16> {
		let ct = self.count() as usize;
		let mut vec = Vec::with_capacity(ct);
		for i in 0 .. ct {
			vec.push( unsafe { &mut *(self.handle().add(i)) } );
		}
		vec
	}
}