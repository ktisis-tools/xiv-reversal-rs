// Dependencies

use libc::c_void;
use std::str::from_utf8;

use memory::MemRegion;

// SigScanner

pub struct SigScanner {
	region: MemRegion
}

impl SigScanner {
	pub fn new(region: MemRegion) -> Self {
		Self { region }
	}

	pub unsafe fn scan_bytes(&self, bytes: &[Option<u8>]) -> Option<*mut c_void> { // cursed
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

			return Some(self.region.base.add(i) as _);
		}

		return None;
	}

	pub fn scan(&self, text: &str) -> Option<*mut c_void> {
		let bytes_str: Vec<&str> = text.split(" ").collect();

		let bytes: Vec<Option<u8>> = bytes_str.into_iter().map(|x| {
			if x == "??" {
				None
			} else {
				Some(u8::from_str_radix(x, 16).unwrap())
			}
		}).collect();
		
		unsafe { self.scan_bytes(&bytes) }
	}
}