use super::*;

pub type Coord = i16;
pub type Dir = aoc_pos::Dir2d;
pub type Grid = aoc_grid::GridBuf <Vec <Tile>, Pos, 2>;
pub type Pos = aoc_pos::PosYX <Coord>;
pub type Pos3 = aoc_pos::PosXYZ <i8>;

enum_decl_parser_display! {

	#[ derive (Clone, Copy, Debug) ]
	pub enum Step {
		Forwards (num: Coord) = [ num ],
		Left = [ "L" ],
		Right = [ "R" ],
	}

	#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
	pub enum Tile {
		#[ default ]
		None = [ " " ],
		Open = [ "." ],
		Wall = [ "#" ],
	}

}
