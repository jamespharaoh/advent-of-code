use super::*;

pub type Coord = i16;
pub type Dir = aoc_pos::Dir2d;
pub type Grid = aoc_grid::GridBuf <Vec <Tile>, Pos, 2>;
pub type Pos = aoc_pos::PosYX <Coord>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
	pub enum Tile {
		Path = [ "." ],
		#[ default ]
		Forest = [ "#" ],
		SlopeUp = [ "^" ],
		SlopeRight = [ ">" ],
		SlopeDown = [ "v" ],
		SlopeLeft = [ "<" ],
	}
}

impl Tile {
	pub fn dir (self) -> Option <Dir> {
		match self {
			Self::Path => None,
			Self::Forest => None,
			Self::SlopeUp => Some (Dir::Up),
			Self::SlopeRight => Some (Dir::Right),
			Self::SlopeDown => Some (Dir::Down),
			Self::SlopeLeft => Some (Dir::Left),
		}
	}
}
