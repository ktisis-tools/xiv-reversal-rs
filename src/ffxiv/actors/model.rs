// Dependnecies

use crate::Vec3;

use std::ffi::c_void;

// Enums

#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum ModelDataPath {
	MidlanderMasc = 101,
	MidlanderMascChild = 104,
	MidlanderFem = 201,
	MidlanderFemChild = 204,
	HighlanderMasc = 301,
	HighlanderFem = 401,
	ElezenMasc = 501,
	ElezenMascChild = 504,
	ElezenFem = 601,
	ElezenFemChild = 604,
	MiqoteMasc = 701,
	MiqoteMascChild = 704,
	MiqoteFem = 801,
	MiqoteFemChild = 804,
	RoegadynMasc = 901,
	RoegadynFem = 1001,
	LalafellMasc = 1101,
	LalafellFem = 1201,
	AuRaMasc = 1301,
	AuRaFem = 1401,
	HrothgarMasc = 1501,
	HrothgarFem = 1601,
	VieraMasc = 1701,
	VieraFem = 1801,
	PadjalMasc = 9104,
	PadjalFem = 9204
}

// ActorModel

#[struct_layout::explicit(size = 0x93c, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct ActorModel {
	#[field(offset = 0x148, get, set)]
	pub bust: *mut BustScale,
	#[field(offset = 0x26c, get, set)]
	pub height: f32,
	#[field(offset = 0x2b0, get, set)]
	pub wetness: f32,
	#[field(offset = 0x2bc, get, set)]
	pub drenched: f32,
	#[field(offset = 0x938, get, set)]
	pub data_path: ModelDataPath
}

impl ActorModel {
	pub fn get_bust(&self) -> &mut BustScale {
		unsafe { &mut *self.bust() }
	}
}

// BustScale

#[struct_layout::explicit(size = 0x93c, align = 4)]
#[derive(Copy, Clone, Debug)]
pub struct BustScale {
	#[field(offset = 0x68, get, set)]
	scale: Vec3
}