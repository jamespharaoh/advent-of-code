use super::*;

pub type Coord = i8;
pub type Tiles = GridBuf <Vec <Tile>, Pos, 2>;
pub type TilesCursor = GridCursor <Pos, 2>;
pub type Pos = pos::PosYX <Coord>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		Floor = [ "." ],
		Empty = [ "L" ],
		Occupied = [ "#" ],
	}
}
