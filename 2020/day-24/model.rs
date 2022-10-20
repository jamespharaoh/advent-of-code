use super::*;

pub type Coord = i16;
pub type Dir = aoc_pos::DirGeoHexLat;
pub type Grid = GridBuf <Vec <Tile>, Pos, 2>;
pub type Pos = aoc_pos::PosGeoHexLat <Coord>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Step {
		East = [ "e" ],
		SouthEast = [ "se" ],
		SouthWest = [ "sw" ],
		West = [ "w" ],
		NorthWest = [ "nw" ],
		NorthEast = [ "ne" ],
	}
}

#[ allow (clippy::fallible_impl_from) ]
impl From <Step> for Pos {
	fn from (step: Step) -> Self {
		match step {
			Step::East => Self::ZERO.east (1).unwrap (),
			Step::SouthEast => Self::ZERO.south_east (1).unwrap (),
			Step::SouthWest => Self::ZERO.south_west (1).unwrap (),
			Step::West => Self::ZERO.west (1).unwrap (),
			Step::NorthWest => Self::ZERO.north_west (1).unwrap (),
			Step::NorthEast => Self::ZERO.north_east (1).unwrap (),
		}
	}
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		White = [ "  " ],
		Black = [ "⬢ " ],
	}
}

impl aoc_bitvec::BitVecNative for Tile {
	const BITS: u32 = 1;
	#[ inline ]
	fn encode (self) -> usize {
		match self {
			Self::White => 0b_0_usize,
			Self::Black => 0b_1_usize,
		}
	}
	#[ inline ]
	fn decode (val: usize) -> Self {
		match val {
			0b_0_usize => Self::White,
			0b_1_usize => Self::Black,
			_ => unreachable! (),
		}
	}
}
