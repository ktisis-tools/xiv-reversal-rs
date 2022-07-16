// Dependencies

use process::Module;

// Module interface

pub struct Dxgi {
	inner: Module
}

impl Dxgi {
	pub fn new(inner: Module) -> Self {
		Self { inner }
	}
}