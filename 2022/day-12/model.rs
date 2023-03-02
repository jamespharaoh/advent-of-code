use super::*;

pub type Coord = i16;
pub type Grid = GridBuf <Vec <Square>, Pos, 2>;
pub type Height = u8;
pub type Pos = aoc_pos::PosYX <Coord>;

#[ derive (Clone, Copy, Eq, PartialEq) ]
pub enum Square {
	Regular { height: Height },
	Start,
	End,
}

impl Square {

	#[ must_use ]
	pub fn height (self) -> Height {
		match self {
			Self::Regular { height } => height,
			Self::Start => 0,
			Self::End => 25,
		}
	}

}

impl Default for Square {

	fn default () -> Self {
		Self::Regular { height: 0 }
	}

}

impl Display for Square {

	fn fmt (& self, fmtr: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Regular { height } =>
				write! (fmtr, "{}", (height.pan_u32 () + 'a'.pan_u32 ()).pan_char ()) ?,
			Self::Start => write! (fmtr, "S") ?,
			Self::End => write! (fmtr, "E") ?,
		}
		Ok (())
	}

}

impl <'inp> FromParser <'inp> for Square {

	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		Ok (match parser.expect_next () ? {
			ch @ 'a' ..= 'z' => Self::Regular { height: (ch.pan_u32 () - 'a'.pan_u32 ()).pan_u8 () },
			'S' => Self::Start,
			'E' => Self::End,
			_ => return Err (parser.err ()),
		})
	}

}
