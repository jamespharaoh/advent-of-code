use super::*;

pub type Coord = i16;
pub type Pos = aoc_pos::PosGeo <Coord>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
	pub enum VHexDir {
		NorthWest = [ "nw" ],
		NorthEast = [ "ne" ],
		SouthWest = [ "sw" ],
		SouthEast = [ "se" ],
		North = [ "n" ],
		South = [ "s" ],
	}
}
