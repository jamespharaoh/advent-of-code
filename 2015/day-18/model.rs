//! Representation of the puzzle input, etc.

use super::*;

pub type Coord = i8;
pub type Pos = aoc_pos::PosYX <Coord>;
pub type Grid = GridBuf <Vec <Light>, Pos, 2>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Light {
		#[ default ]
		Off = [ "." ],
		On = [ "#" ],
	}
}
