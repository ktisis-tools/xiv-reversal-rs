// Dependencies

use crate::{
	memory::MemRegion,
	ActorModel
};

use std::{
	ffi::{
		c_void,
		CStr, CString
	},
	mem::size_of,
	time::Duration
};

use async_std::task;
use futures::executor::block_on;

// Enums

#[repr(u32)]
#[derive(Debug)]
pub enum RenderMode {
	Draw = 0,
	Unload = 2,
	Load = 4
}

#[repr(u32)]
#[derive(PartialEq, Debug)]
pub enum ActorType {
	None = 0x0,
	Player = 0x1,
	BattleNpc = 0x2,
	EventNpc = 0x3,
	Trasure = 0x4,
	Aetheryte = 0x5,
	GatheringPoint = 0x6,
	EventObj = 0x7,
	Mount = 0x8,
	Minion = 0x9,
	Retainer = 0xA,
	Area = 0xB,
	Housing = 0xC,
	Cutscene = 0xD,
	CardStand = 0xE,
	Ornament = 0xF
}

// Offsets

#[repr(usize)]
enum _Offset {
	Name = 0x30,
	ActorType = 0x8c,
	ActorModel = 0xf0,
	RenderMode = 0x104
}

// Actor

#[derive(Debug)]
pub struct Actor {
	index: u32,
	handle: *mut c_void
}

impl Actor {
	pub fn new(index: u32, handle: *mut c_void) -> Self {
		Self { index, handle }
	}

	pub fn get<T>(&self, offset: _Offset) -> &mut T {
		unsafe { &mut *( self.handle.add(offset as usize) as *mut T ) }
	}

	/*	this needs to be more intuitive
		like just Actor.whatever would be most ideal
		maybe make a macro for this? */

	pub fn get_name(&self) -> String {
		unsafe {
			CStr::from_ptr( self.handle.add(_Offset::Name as _) as *mut i8 )
			.to_str().unwrap().to_string()
		}
	}

	pub fn get_actor_type(&self) -> &mut ActorType {
		self.get( _Offset::ActorType )
	}

	pub fn get_render_mode(&self) -> &mut RenderMode {
		self.get( _Offset::RenderMode )
	}

	// ActorModel

	pub fn get_model(&self) -> ActorModel {
		unsafe { ActorModel::new( self.handle.add(_Offset::ActorModel as _) ) }
	}

	// Redraw Actor

	pub async fn redraw(&self) {
		let mode = self.get_render_mode();
		let actor_type = self.get_actor_type();

		let switch = *actor_type == ActorType::Player;

		if switch { *actor_type = ActorType::BattleNpc }
		*mode = RenderMode::Unload;
		task::sleep( Duration::from_millis(75) ).await;
		*mode = RenderMode::Draw;
		if switch { *actor_type = ActorType::Player }
	}

	pub fn redraw_sync(&self) {
		block_on(self.redraw());
	}
}