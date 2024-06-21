use super::*;

pub type Coord = i8;
pub type Dir = aoc_pos::Dir2d;
pub type Grid = aoc_grid::GridBuf <Vec <Tile>, Pos, 2>;
pub type Pos = aoc_pos::PosRowCol <Coord>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		Empty = [ "." ],
		RoundRock = [ "O" ],
		CubeRock = [ "#" ],
	}
}
