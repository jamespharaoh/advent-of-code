use super::*;

pub type Coord = i8;
pub type Dir = pos::Dir2d;
pub type Grid = GridBuf <Vec <Tile>, Pos, 2>;
pub type Pos = pos::PosYX <Coord>;
pub type Turn = pos::Turn2d;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		Open = [ "." ],
		DeadEnd = [ "~" ],
		Wall = [ "#" ],
		Entrance = [ "@" ],
		Key (id: u8) = [
			@display { let id = (id + b'a').pan_char (); },
			id = 'a' ..= 'z',
			@parse { let id = id.pan_u8 () - b'a'; },
		],
		Door (id: u8) = [
			@display { let id = (id + b'A').pan_char (); },
			id = 'A' ..= 'Z',
			@parse { let id = id.pan_u8 () - b'A'; },
		],
	}
}
