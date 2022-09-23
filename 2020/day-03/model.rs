use super::*;

pub type Coord = i16;
pub type Grid = aoc_grid::Grid <Vec <Tile>, Pos>;
pub type Pos = aoc_pos::PosYX <Coord>;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		Open = ".",
		Tree = "#",
	}
}
