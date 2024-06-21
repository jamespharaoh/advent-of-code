use super::*;

pub type Coord = i8;
pub type Dir = aoc_pos::Dir2d;
pub type Grid <Val> = aoc_grid::GridBuf <Vec <Val>, Pos, 2>;
pub type Pos = aoc_pos::PosYX <Coord>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
	pub enum Tile {
		#[ default ]
		Empty = [ "." ],
		MirrorBack = [ "\\" ],
		MirrorForward = [ "/" ],
		SplitterVertical = [ "|" ],
		SplitterHorizontal = [ "-" ],
	}
}
