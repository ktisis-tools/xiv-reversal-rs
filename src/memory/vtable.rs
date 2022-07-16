// VTable

#[derive(Copy, Clone, Debug)]
pub struct VTable {
	pub handle: *const usize,
	pub methods: usize
}

impl VTable {
	pub fn new(handle: *const usize) -> Self {
		let methods = unsafe {
			let mut count = 0;
			while !(handle.offset(count).read() as *const usize).is_null() {
				count += 1;
			}
			count as usize
		};

		Self {
			handle,
			methods
		}
	}

	pub fn raw_vtable(self) -> *const usize {
		unsafe { *(self.handle as *const *const usize) }
	}

	pub fn nth(self, index: isize) -> *const usize {
		unsafe {
			self.raw_vtable().offset(index).read() as *const usize
		}
	}
}