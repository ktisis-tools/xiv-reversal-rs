// Dependencies

use std::ffi::CString;

use winapi::{
	um::libloaderapi::GetProcAddress,
	shared::minwindef::{
		HMODULE,
		FARPROC
	}
};

use libc::c_void;

use memory::MemRegion;

// Module

#[derive(Copy, Clone)]
pub struct Module {
	pub handle: HMODULE,
	pub memory: MemRegion
}

impl Module {
	pub fn new(handle: HMODULE, base: *mut c_void, size: usize) -> Self {
		let memory = MemRegion::new(base, size);
		Self {
			handle,
			memory
		}
	}

	pub fn get_export(&mut self, name: &str) -> Option<FARPROC> {
		let proc = unsafe {
			GetProcAddress(
				self.handle,
				CString::new(name).unwrap().as_bytes_with_nul().as_ptr() as *const _
			)
		};

		if proc.is_null() {
			None
		} else {
			Some(proc)
		}
	}
}