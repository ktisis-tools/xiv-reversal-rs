// Dependencies

use std::ptr::null_mut;
use libc::c_void;

use crate::memory::MemRegion;

// SigScanner

pub struct SigScanner {
	region: MemRegion
}

impl SigScanner {
	pub fn new(region: MemRegion) -> Self {
		Self { region }
	}

	pub unsafe fn scan_bytes(&self, bytes: &[Option<u8>]) -> SigScanResult { // cursed
		let len = bytes.len();
		
		'a: for i in 0 .. (self.region.size as usize - len) {
			for (x, byte) in bytes.iter().enumerate() {
				let byte = match byte {
					Some(s) => *s,
					None => continue
				};

				if *(self.region.base.add(i + x) as *const u8) != byte {
                    continue 'a;
                }
			}

			return SigScanResult {
				ptr: self.region.base.add(i) as _,
				len: bytes.len(),
				query: bytes.to_vec()
			}
		}

		SigScanResult::null()
	}

	pub fn scan(&self, text: &str) -> SigScanResult {
		let bytes_str: Vec<&str> = text.split(" ").collect();

		let bytes: Vec<Option<u8>> = bytes_str.into_iter().map(|x| {
			if x == "?" || x == "??" {
				None
			} else {
				Some(u8::from_str_radix(x, 16).unwrap())
			}
		}).collect();
		
		unsafe { self.scan_bytes(&bytes) }
	}
}

// SigScanResult

pub struct SigScanResult {
	ptr: *mut c_void,
	len: usize,
	query: Vec<Option<u8>>
}

impl SigScanResult {
	pub fn null() -> Self {
		Self {
			ptr: null_mut(),
			len: 0,
			query: vec![]
		}
	}

	pub fn is_null(&self) -> bool {
		self.ptr.is_null()
	}

	pub fn unwrap(&self) -> *mut c_void {
		self.ptr
	}

	pub fn asm_ptr(&self) -> *mut c_void {
		// Get ptr offset

		let mut offset = 0;
		let mut found = false;
		for (i, byte) in self.query.iter().enumerate() {
			if byte.is_none() {
				offset = i;
				found = true;
				break;
			}
		}

		if !found {
			println!("Ptr offset not found in query: {:?}", self.query);
			return null_mut();
		}

		// Resolve ASM ptr

		unsafe {
			println!("-> {:x?}", self.ptr);
			let asm_ptr = *(self.ptr.add(offset) as *const u32);
			self.ptr.add(asm_ptr as usize + offset*2 + 1)
		}
	}
}