use super::*;

pub type Coord = i8;
pub type PosXY = pos::PosXY <Coord>;
pub type PosXYZ = pos::PosXYZ <Coord>;
pub type PosXYZW = pos::PosXYZW <Coord>;
pub type Grid <Pos, const DIMS: usize> = GridBuf <Vec <Tile>, Pos, DIMS>;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		Inactive = ".",
		Active = "#",
	}
}

pub trait GenPos <const DIMS: usize>:
	pos::GenPos <DIMS, Val = Coord> +
	GridPos <DIMS, Coord = Coord> {
}

impl <Pos, const DIMS: usize> GenPos <DIMS> for Pos where Pos:
	pos::GenPos <DIMS, Val = Coord> +
	GridPos <DIMS, Coord = Coord> {
}
