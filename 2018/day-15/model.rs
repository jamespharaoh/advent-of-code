use super::*;

pub type Coord = u8;
pub type Dir = pos::Dir2d;
pub type Grid = GridBuf <Vec <Tile>, Pos, 2>;
pub type SeenGrid = GridBuf <Vec <bool>, Pos, 2>;
pub type Pos = pos::PosYX <Coord>;

enum_decl_parser_display! {

	#[ derive (Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		Open = [ "." ],
		Cavern = [ "#" ],
		Goblin = [ "G" ],
		Elf = [ "E" ],
	}

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
