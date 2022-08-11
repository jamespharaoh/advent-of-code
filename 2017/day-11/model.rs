use super::*;

pub type Coord = i16;
pub type Pos = pos::PosGeo <Coord>;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
	pub enum VHexDir {
		NorthWest = "nw",
		NorthEast = "ne",
		SouthWest = "sw",
		SouthEast = "se",
		North = "n",
		South = "s",
	}
}
