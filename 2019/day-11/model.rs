use super::*;

pub type Coord = i16;
pub type Cpu = intcode::Machine <Val>;
pub type Dir = aoc_pos::Dir2d;
pub type Grid = GridBuf <Vec <Colour>, Pos, 2>;
pub type Pos = aoc_pos::PosYX <Coord>;
pub type Val = i64;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Colour {
		#[ default ]
		None = [ " " ],
		Black = [ "." ],
		White = [ "#" ],
	}
}
