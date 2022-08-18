use super::*;

pub type Coord = u8;
pub type Dir = pos::Dir2d;
pub type Grid = grid::Grid <Vec <Tile>, Pos>;
pub type SeenGrid = grid::Grid <Vec <bool>, Pos>;
pub type Pos = pos::PosYX <Coord>;

parse_display_enum! {

	#[ derive (Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd) ]
	pub enum Tile { #[default] Open = ".", Cavern = "#", Goblin = "G", Elf = "E" }

}

impl Tile {

	#[ must_use ]
	pub fn enemy (self) -> Self {
		match self {
			Self::Goblin => Self::Elf,
			Self::Elf => Self::Goblin,
			Self::Open | Self::Cavern => panic! (),
		}
	}

}
