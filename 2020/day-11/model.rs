use super::*;

pub type Coord = i8;
pub type Grid = grid::Grid <Vec <Tile>, Pos>;
pub type GridCursor <'sto> = grid::GridCursor <'sto, Vec <Tile>, Pos>;
pub type Pos = pos::PosYX <Coord>;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		Floor = ".",
		Empty = "L",
		Occupied = "#",
	}
}
