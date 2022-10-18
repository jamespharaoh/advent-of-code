use super::*;

pub type Coord = i16;
pub type Dir = aoc_pos::Dir2d;
pub type Grid = GridBuf <Vec <Node>, Pos, 2>;
pub type Pos = aoc_pos::PosRowCol <Coord>;
pub type Turn = aoc_pos::Turn2d;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
	pub enum Node {
		#[ default ]
		Clean = [ "." ],
		Weakened = [ "W" ],
		Infected = [ "#" ],
		Flagged = [ "F" ],
	}
}
