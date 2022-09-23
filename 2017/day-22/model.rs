use super::*;

pub type Coord = i16;
pub type Dir = aoc_pos::Dir2d;
pub type Grid = aoc_grid::Grid <Vec <Node>, Pos>;
pub type Pos = aoc_pos::PosRowCol <Coord>;
pub type Turn = aoc_pos::Turn2d;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Node { Clean = ".", Weakened = "W", Infected = "#", Flagged = "F" }
}
