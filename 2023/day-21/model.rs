use super::*;

pub type Coord = i32;
pub type Dir = aoc_pos::DirGeo;
pub type Grid <Val> = GridBuf <Vec <Val>, Pos, 2>;
pub type Pos = aoc_pos::PosGeo <Coord>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
	pub enum Tile {
		#[ default ]
		Garden = [ "." ],
		Rock = [ "#" ],
		Start = [ "S" ],
	}
}
