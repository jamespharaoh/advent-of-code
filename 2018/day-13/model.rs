//! Data representation and algorithms used to solve the puzzle

use super::*;

pub type Coord = u16;
pub type Dir = pos::Dir2d;
pub type Grid = GridBuf <Vec <Tile>, Pos, 2>;
pub type Pos = pos::PosYX <Coord>;
pub type Turn = pos::Turn2d;

parse_display_enum! {

	#[ derive (Clone, Copy, Debug, Default) ]
	pub enum Tile {
		#[default] Empty = " ",
		Vert = "|",
		Horiz = "-",
		Crossing = "+",
		CornerFwd = "/",
		CornerBck = "\\",
		CartUp = "^",
		CartDown = "v",
		CartLeft = "<",
		CartRight = ">",
	}

}

impl Tile {

	#[ inline ]
	#[ must_use ]
	pub const fn is_cart (self) -> bool {
		matches! (self, Self::CartUp | Self::CartDown | Self::CartLeft | Self::CartRight)
	}

	#[ inline ]
	#[ must_use ]
	pub const fn is_crossing (self) -> bool {
		matches! (self, Self::Crossing)
	}

}
