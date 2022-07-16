#![feature(abi_thiscall)]

// Modules

mod memory;

mod process;
use process::Process;

mod d3d11;

// Dpeendnecies

extern crate winapi;
extern crate libc;

use std::{
	ptr::null_mut,
	panic::catch_unwind
};

use winapi::um::{
	processthreadsapi::CreateThread,
	libloaderapi::{
		DisableThreadLibraryCalls,
		FreeLibraryAndExitThread
	},
	winnt::{
		DLL_PROCESS_ATTACH,
		DLL_PROCESS_DETACH
	},
	wincon::{
		FreeConsole,
		SetConsoleTitleA
	},
	consoleapi::AllocConsole
};

use winapi::shared::minwindef::{
	HINSTANCE,
	DWORD,
	LPVOID
};

// DLL attachment

fn dll_attach(_lpv: LPVOID) {
	unsafe {
		AllocConsole();
		SetConsoleTitleA("Debug Console\0".as_ptr() as *const _);
	}

	let process = Process::get();

	println!(
		"Attached to process.\n\
		- PID: {}\n\
		- Address space: {:x?} - 0x{:x?} (size of 0x{:x?})",
		process.pid,
		process.memory.base,
		process.memory.base as usize + process.memory.size,
		process.memory.size
	);
}

fn dll_detach(lpv: LPVOID) {
	unsafe {
		FreeConsole();
		FreeLibraryAndExitThread(lpv as _, 1);
	}
}

// Spawn thread

unsafe extern "system" fn spawn_thread(lpv: LPVOID) -> u32 {
	// TODO: Result handling

	catch_unwind(|| dll_attach(lpv)).ok();

	println!("Press enter to detach.");
	std::io::stdin().read_line(&mut "".to_owned()).ok();

	catch_unwind(|| dll_detach(lpv)).ok();

	return 1;
}

// Entry function

#[no_mangle]
pub extern "system" fn DllMain(inst: HINSTANCE, reason: DWORD, lpv: LPVOID) {
	match reason {
		DLL_PROCESS_ATTACH => {
			unsafe {
				DisableThreadLibraryCalls(inst);
				CreateThread(null_mut(), 0, Some(spawn_thread), inst as _, 0, null_mut());
			}
		},
		DLL_PROCESS_DETACH => if !lpv.is_null() {
			catch_unwind(|| dll_detach(lpv)).ok();
		},
		_ => ()
	}
}