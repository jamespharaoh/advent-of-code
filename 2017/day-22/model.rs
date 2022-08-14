use super::*;

pub type Coord = i16;
pub type Dir = pos::Dir2d;
pub type Grid = grid::Grid <Vec <Node>, Pos>;
pub type Pos = pos::PosRowCol <Coord>;
pub type Turn = pos::Turn2d;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Node { Clean = ".", Weakened = "W", Infected = "#", Flagged = "F" }
}
