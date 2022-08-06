// Dependencies

use crate::{
	Ktisis,
	render::d3d11::Device
};

use std::mem::transmute;

use winapi::{
	um::winnt::HRESULT,
	shared::dxgi::IDXGISwapChain
};

// Consts

type Present = unsafe extern "stdcall" fn(*mut IDXGISwapChain, u32, u32) -> HRESULT;
static mut ORIGIN: Option<Present> = None;

static mut KTISIS: Option<*mut Ktisis> = None;

// Present

unsafe extern "stdcall" fn present(
	swap_chain: *mut IDXGISwapChain,
	sync_interval: u32,
	flags: u32
) -> HRESULT {
	if let Some(ktisis) = KTISIS {
		/*let ktisis = &mut *ktisis;

		let matrix = ktisis.game.get_world_matrix();

		ktisis.overlay.draw(&|ui: &Ui| {
			let draw = ui.get_foreground_draw_list();

			//let player = &ktisis.game.actor_table.get_all()[0];
			for player in &ktisis.game.actor_table.get_all() {
				let mut pl_pos = player.position();

				if player.model().is_null() {
					continue;
				}

				let model = player.get_model();

				let skeletons = model.get_skeleton().get_vec();

				for skeleton in skeletons {
					for pose in skeleton.poses() {
						if pose.is_null() { continue; }
						let mut pose = unsafe { *pose };
		
						let skelly = pose.get_skeleton();
		
						let mut transforms = pose.transforms();
						let mut trans_vec = transforms.get_vec();
		
						let mut i = 0;
						for bone in skelly.bones().get_vec() {
							let mut trans = &mut trans_vec[i];
		
							let name = bone.get_name();
							if name == "j_kosi" {
								let mut translate = trans.translate();

								pl_pos.x += translate.x;
								pl_pos.y += translate.y;
								pl_pos.z += translate.z;

								for radius in 1 .. 10 {
									draw.add_circle(
										matrix.world_to_screen(pl_pos).to_vec(),
										radius as f32,
										[1.0, 1.0, 1.0]
									).build();
								}
							}
		
							i += 1;
						}
					}
				}
			}
		});*/
	};
	
	if let Some(call) = &ORIGIN {
		call(swap_chain, sync_interval, flags)
	} else { 0 }
}

// Init hook

pub fn init(ktisis: &mut Ktisis) {
	let device = Device::from(&ktisis.process);

	let sc_vt = device.get_swapchain_vt();

	// this needs to be more intuitive
	unsafe {
		KTISIS = Some(ktisis);

		let present_fn = transmute::<_, Present>(
			sc_vt.nth(8)
		);
		ORIGIN = Some(present_fn);
		
		let hook = ktisis.hooks.hook_vt(
			sc_vt.raw_vtable().offset(8) as *mut usize,
			present as _
		);

		println!("Hook created. Replacing VTable entry at {:x?} with {:x?} (originally {:x?})", hook.handle, hook.hook, hook.original);

		hook.enable();
	}
}