// Dependencies

extern crate dll_syringe;
use dll_syringe::{Syringe, process::OwnedProcess};

// Execute

fn main() {
	let exists = OwnedProcess::find_first_by_name("ffxiv_dx11.exe");
	if let Some(target) = exists {
		println!("Injecting into process...");

		let syringe = Syringe::for_process(target);
		if let Err(err) = syringe.inject("gpose_plus.dll") {
			println!("Injection failed: {err}");
		} else {
			println!("Success");
		}
	} else {
		println!("Process not found.");
	}
}