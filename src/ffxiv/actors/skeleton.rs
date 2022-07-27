use std::ffi::CStr;
use libc::c_char;

// Consts

const SKELETON_SIZE: usize = 448;
const BONE_SIZE: usize = 16;

const POSE_CT: usize = 4;

/*
	Memory Arrays
	TODO: MACRO THIS
*/

// SkeletonArray

#[struct_layout::explicit(size = 0x70, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct SkeletonArray {
	#[field(offset = 0x50, get, set)]
	pub count: u16,
	#[field(offset = 0x68, get, set)]
	pub handle: *mut Skeleton
}

impl SkeletonArray {
	pub fn get_vec(&self) -> Vec<&mut Skeleton> {
		let ct = self.count() as usize;
		let mut vec = Vec::with_capacity(ct);
		for i in 0 .. ct {
			vec.push( unsafe { &mut *(self.handle().add(SKELETON_SIZE * i)) } );
		}
		vec
	}
}

// TODO: BoneArray, ParentArray

// Skeleton

#[struct_layout::explicit(size = 0x160, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct Skeleton {
	#[field(offset = 0x12c, get, set)]
	pub bone_node_index: u16,
	#[field(offset = 0x12e, get, set)]
	pub bone_parent_index: u16,

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

#[struct_layout::explicit(size = 0x8, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct HkaPose {
	#[field(offset = 0x0, get, set)]
	pub skeleton: *mut HkaSkeleton
	// TODO: transforms
}

impl HkaPose {
	pub fn get_skeleton(&self) -> &mut HkaSkeleton {
		unsafe { &mut *self.skeleton() }
	}
}

// HkaSkeleton

#[struct_layout::explicit(size = 0x18, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct HkaSkeleton {
	#[field(offset = 0x10, get, set)]
	pub name: *mut c_char
}

impl HkaSkeleton {
	pub fn get_name(&self) -> String {
		unsafe { CStr::from_ptr(self.name()).to_str().unwrap().to_string() }
	}
}