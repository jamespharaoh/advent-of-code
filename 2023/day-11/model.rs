use super::*;

pub type Coord = i64;
pub type Grid = aoc_grid::GridBuf <Vec <Tile>, Pos, 2>;
pub type Pos = aoc_pos::PosXY <Coord>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
	pub enum Tile {
		#[ default ]
		Empty = [ "." ],
		Galaxy = [ "#" ],
	}
}
