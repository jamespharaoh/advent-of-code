use super::*;

pub type Coord = i8;
pub type PosXY = pos::PosXY <Coord>;
pub type PosXYZ = pos::PosXYZ <Coord>;
pub type PosXYZW = pos::PosXYZW <Coord>;
pub type Grid <Pos, const DIMS: usize> = GridBuf <Vec <Tile>, Pos, DIMS>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		Inactive = [ "." ],
		Active = [ "#" ],
	}
}

pub trait GenPos <const DIMS: usize>: pos::GenPos <DIMS, Val = Coord>
		+ GridPos <DIMS, Coord = Coord>
		+ 'static{
	const BASE_DIRS: & 'static [Self];
}

impl GenPos <2> for PosXY {
	const BASE_DIRS: & 'static [Self] = & [
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::ZERO, Coord::NEG_ONE),
		//Self::new (Coord::ZERO, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::ONE),
		Self::new (Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::ZERO),
		Self::new (Coord::ONE, Coord::ONE),
	];
}

impl GenPos <3> for PosXYZ {
	const BASE_DIRS: & 'static [Self] = & [
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::NEG_ONE, Coord::ZERO, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::ZERO, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::ZERO, Coord::ONE),
		Self::new (Coord::NEG_ONE, Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::ONE, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::ONE, Coord::ONE),
		Self::new (Coord::ZERO, Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::ZERO, Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::ZERO, Coord::ZERO, Coord::NEG_ONE),
		//Self::new (Coord::ZERO, Coord::ZERO, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::ZERO, Coord::ONE),
		Self::new (Coord::ZERO, Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::ZERO, Coord::ONE, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::ONE, Coord::ONE),
		Self::new (Coord::ONE, Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::ONE, Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::ONE, Coord::ZERO, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::ZERO, Coord::ZERO),
		Self::new (Coord::ONE, Coord::ZERO, Coord::ONE),
		Self::new (Coord::ONE, Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::ONE, Coord::ZERO),
		Self::new (Coord::ONE, Coord::ONE, Coord::ONE),
	];
}

impl GenPos <4> for PosXYZW {
	const BASE_DIRS: & 'static [Self] = & [
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::ZERO, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::ZERO, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::ZERO, Coord::ONE),
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::ONE, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::NEG_ONE, Coord::ONE, Coord::ONE),
		Self::new (Coord::NEG_ONE, Coord::ZERO, Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::ZERO, Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::ZERO, Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::NEG_ONE, Coord::ZERO, Coord::ZERO, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::ZERO, Coord::ZERO, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::ZERO, Coord::ZERO, Coord::ONE),
		Self::new (Coord::NEG_ONE, Coord::ZERO, Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::ZERO, Coord::ONE, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::ZERO, Coord::ONE, Coord::ONE),
		Self::new (Coord::NEG_ONE, Coord::ONE, Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::ONE, Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::ONE, Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::NEG_ONE, Coord::ONE, Coord::ZERO, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::ONE, Coord::ZERO, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::ONE, Coord::ZERO, Coord::ONE),
		Self::new (Coord::NEG_ONE, Coord::ONE, Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::NEG_ONE, Coord::ONE, Coord::ONE, Coord::ZERO),
		Self::new (Coord::NEG_ONE, Coord::ONE, Coord::ONE, Coord::ONE),
		Self::new (Coord::ZERO, Coord::NEG_ONE, Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::ZERO, Coord::NEG_ONE, Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::NEG_ONE, Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::ZERO, Coord::NEG_ONE, Coord::ZERO, Coord::NEG_ONE),
		Self::new (Coord::ZERO, Coord::NEG_ONE, Coord::ZERO, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::NEG_ONE, Coord::ZERO, Coord::ONE),
		Self::new (Coord::ZERO, Coord::NEG_ONE, Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::ZERO, Coord::NEG_ONE, Coord::ONE, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::NEG_ONE, Coord::ONE, Coord::ONE),
		Self::new (Coord::ZERO, Coord::ZERO, Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::ZERO, Coord::ZERO, Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::ZERO, Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::ZERO, Coord::ZERO, Coord::ZERO, Coord::NEG_ONE),
		//Self::new (Coord::ZERO, Coord::ZERO, Coord::ZERO, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::ZERO, Coord::ZERO, Coord::ONE),
		Self::new (Coord::ZERO, Coord::ZERO, Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::ZERO, Coord::ZERO, Coord::ONE, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::ZERO, Coord::ONE, Coord::ONE),
		Self::new (Coord::ZERO, Coord::ONE, Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::ZERO, Coord::ONE, Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::ONE, Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::ZERO, Coord::ONE, Coord::ZERO, Coord::NEG_ONE),
		Self::new (Coord::ZERO, Coord::ONE, Coord::ZERO, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::ONE, Coord::ZERO, Coord::ONE),
		Self::new (Coord::ZERO, Coord::ONE, Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::ZERO, Coord::ONE, Coord::ONE, Coord::ZERO),
		Self::new (Coord::ZERO, Coord::ONE, Coord::ONE, Coord::ONE),
		Self::new (Coord::ONE, Coord::NEG_ONE, Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::NEG_ONE, Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::ONE, Coord::NEG_ONE, Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::ONE, Coord::NEG_ONE, Coord::ZERO, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::NEG_ONE, Coord::ZERO, Coord::ZERO),
		Self::new (Coord::ONE, Coord::NEG_ONE, Coord::ZERO, Coord::ONE),
		Self::new (Coord::ONE, Coord::NEG_ONE, Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::NEG_ONE, Coord::ONE, Coord::ZERO),
		Self::new (Coord::ONE, Coord::NEG_ONE, Coord::ONE, Coord::ONE),
		Self::new (Coord::ONE, Coord::ZERO, Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::ZERO, Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::ONE, Coord::ZERO, Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::ONE, Coord::ZERO, Coord::ZERO, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::ZERO, Coord::ZERO, Coord::ZERO),
		Self::new (Coord::ONE, Coord::ZERO, Coord::ZERO, Coord::ONE),
		Self::new (Coord::ONE, Coord::ZERO, Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::ZERO, Coord::ONE, Coord::ZERO),
		Self::new (Coord::ONE, Coord::ZERO, Coord::ONE, Coord::ONE),
		Self::new (Coord::ONE, Coord::ONE, Coord::NEG_ONE, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::ONE, Coord::NEG_ONE, Coord::ZERO),
		Self::new (Coord::ONE, Coord::ONE, Coord::NEG_ONE, Coord::ONE),
		Self::new (Coord::ONE, Coord::ONE, Coord::ZERO, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::ONE, Coord::ZERO, Coord::ZERO),
		Self::new (Coord::ONE, Coord::ONE, Coord::ZERO, Coord::ONE),
		Self::new (Coord::ONE, Coord::ONE, Coord::ONE, Coord::NEG_ONE),
		Self::new (Coord::ONE, Coord::ONE, Coord::ONE, Coord::ZERO),
		Self::new (Coord::ONE, Coord::ONE, Coord::ONE, Coord::ONE),
	];
}
