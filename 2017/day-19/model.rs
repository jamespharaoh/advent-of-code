use super::*;

pub type Coord = u8;
pub type Dir = pos::Dir2d;
pub type Grid = grid::Grid <Vec <Tile>, Pos>;
pub type Pos = pos::PosRowCol <Coord>;
pub type Turn = pos::Turn2d;

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum Tile {
	Empty,
	Horiz,
	Vert,
	Corner,
	Letter (u8),
}

impl Tile {

	#[ must_use ]
	pub fn as_char (& self) -> char {
		match * self {
			Self::Empty => ' ',
			Self::Horiz => '-',
			Self::Vert => '|',
			Self::Corner => '+',
			Self::Letter (asc) => asc.as_char (),
		}
	}

}

impl Display for Tile {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "{}", self.as_char ()) ?;
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for Tile {
	fn from_parser (parser: & mut Parser) -> ParseResult <Self> {
		parser.any ()
			.of (|parser| {
				let ch = parser.expect_next () ?;
				let tile = match ch {
					' ' => Self::Empty,
					'-' => Self::Horiz,
					'|' => Self::Vert,
					'+' => Self::Corner,
					'A' ..= 'Z' => Self::Letter (ch.as_u8 ()),
					_ => return Err (parser.err ()),
				};
				Ok (tile)
			})
			.done ()
	}
}
