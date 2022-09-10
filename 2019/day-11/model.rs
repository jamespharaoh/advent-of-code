use super::*;

pub type Coord = i16;
pub type Cpu = intcode::Machine <Val>;
pub type Dir = pos::Dir2d;
pub type Grid = grid::Grid <Vec <Colour>, Pos>;
pub type Pos = pos::PosYX <Coord>;
pub type Val = i64;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Colour {
		#[ default ]
		None = " ",
		Black = ".",
		White = "#",
	}
}
