// Dependencies

use crate::{
	memory::MemRegion,
	ActorModel
};

use std::{
	time::Duration,
	str::from_utf8_unchecked
};

use async_std::task;
use futures::executor::block_on;

// Enums

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum RenderMode {
	Draw = 0,
	Unload = 2,
	Load = 4
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
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

// Actor

#[struct_layout::explicit(size = 0x20c, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct Actor {
	#[field(offset = 0x30, get, set)]
	pub name_bytes: [u8; 64], // assuming 64 bytes, probably incorrect?
	#[field(offset = 0x8c, get, set)]
	pub actor_type: ActorType,
	#[field(offset = 0x104, get, set)]
	pub render_mode: RenderMode,

	#[field(offset = 0xb4, get, set)]
	pub scale: f32,

	#[field(offset = 0xf0, get, set)] // TODO: ptr
	pub model: *mut ActorModel
}

impl Actor {
	pub fn get_model(&self) -> &mut ActorModel {
		unsafe { &mut *self.model() }
	}

	// Get name

	pub fn get_name(&self) -> String {
		let bytes = &self.name_bytes();
		let x = if let Some(x) = self.name_bytes().iter().position(|x| x == &0) { x } else { bytes.len() };
		unsafe { from_utf8_unchecked(&bytes[0 .. x]).to_string() }
	}

	// Redraw Actor

	pub async fn redraw(&mut self) {
		let switch = self.actor_type() == ActorType::Player;

		if switch { self.set_actor_type(ActorType::BattleNpc); }

		self.set_render_mode(RenderMode::Unload);
		task::sleep( Duration::from_millis(75) ).await;
		self.set_render_mode(RenderMode::Draw);
		
		if switch { self.set_actor_type(ActorType::Player); }
	}

	pub fn redraw_sync(&mut self) {
		block_on(self.redraw());
	}
}