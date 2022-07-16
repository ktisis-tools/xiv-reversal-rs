// Dependencies

use std::{
	mem,
	ptr::null,
	collections::HashMap,
	os::windows::ffi::OsStringExt,
	ffi::{
		OsString,
		c_void
	}
};

use winapi::um::{
	libloaderapi::GetModuleHandleW,
	processthreadsapi::{
		GetProcessId,
		GetCurrentProcess
	},
	tlhelp32::{
		CreateToolhelp32Snapshot as CreateTH32,
		Module32NextW,
		MODULEENTRY32W,
		TH32CS_SNAPMODULE
	},
	handleapi::INVALID_HANDLE_VALUE,
	winnt::HANDLE
};

use process::Module;
use memory::MemRegion;

// Process

pub struct Process {
	pub pid: u32,
	pub handle: HANDLE,
	pub base: *mut c_void,
	pub modules: HashMap<String, Module>,
	pub memory: MemRegion
}

impl Process {
	pub fn get() -> Self {
		let handle = unsafe { GetCurrentProcess() };
		let pid = unsafe { GetProcessId(handle) };
		let base = unsafe { GetModuleHandleW(null()) } as _;
		let modules = Self::get_modules(pid);
		
		let proc_mod = modules.values().find(|x| x.memory.base == base).unwrap();
		let memory = MemRegion::new(base, proc_mod.memory.size);

		Self {
			handle,
			pid,
			base,
			modules,
			memory
		}
	}

	pub fn get_modules(pid: u32) -> HashMap<String, Module> {
		let mut modules = HashMap::<String, Module>::new();

		let handle = unsafe { CreateTH32(TH32CS_SNAPMODULE, pid) };
		assert!(handle != INVALID_HANDLE_VALUE, "TH32 returned invalid handle.");
		
		let mut entry: MODULEENTRY32W = unsafe { mem::zeroed() };
		entry.dwSize = mem::size_of::<MODULEENTRY32W>() as _;

		while unsafe { Module32NextW(handle, &mut entry) } != 0 {
			if let Ok(name) = OsString::from_wide(&entry.szModule).into_string() {
				let name = name.trim_matches('\0').trim().to_string();

				if modules.contains_key(&name) {
					continue;
				}

				let module = Module::new(
					entry.hModule,
					entry.modBaseAddr as _,
					entry.modBaseSize as _
				);

				modules.insert(name, module);
			}
		}

		return modules;
	}
}