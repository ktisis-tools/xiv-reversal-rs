// Dependnecies

use std::ffi::c_void;

// Enums

#[repr(u16)]
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

pub struct ActorModel {}

impl ActorModel {}