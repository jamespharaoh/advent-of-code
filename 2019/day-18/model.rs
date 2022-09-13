use super::*;

pub type Coord = i8;
pub type Dir = pos::Dir2d;
pub type Grid = grid::Grid <Vec <Tile>, Pos>;
pub type Pos = pos::PosYX <Coord>;
pub type Turn = pos::Turn2d;

#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum Tile {
	#[ default ]
	Open,
	DeadEnd,
	Wall,
	Entrance,
	Key (u8),
	Door (u8),
}

impl Display for Tile {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Open => write! (formatter, "."),
			Self::DeadEnd => write! (formatter, "~"),
			Self::Wall => write! (formatter, "#"),
			Self::Entrance => write! (formatter, "@"),
			Self::Key (id) => write! (formatter, "{}", ('a'.as_u8 () + id).as_char ()),
			Self::Door (id) => write! (formatter, "{}", ('A'.as_u8 () + id).as_char ()),
		}
	}
}

impl <'inp> FromParser <'inp> for Tile {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let tile = match parser.peek ().ok_or_else (|| parser.err ()) ? {
			'.' => Self::Open,
			'#' => Self::Wall,
			'@' => Self::Entrance,
			ch @ 'a' ..= 'z' => Self::Key (ch.as_u8 () - 'a'.as_u8 ()),
			ch @ 'A' ..= 'Z' => Self::Door (ch.as_u8 () - 'A'.as_u8 ()),
			_ => return Err (parser.err ()),
		};
		parser.expect_next () ?;
		Ok (tile)
	}
}
