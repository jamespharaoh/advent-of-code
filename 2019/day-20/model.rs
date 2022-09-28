use super::*;

pub type Coord = i8;
pub type Dir = pos::Dir2d;
pub type Grid = GridBuf <Vec <Tile>, Pos, 2>;
pub type Pos = pos::PosYX <Coord>;

#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum Tile {
	#[ default ]
	Empty,
	Passage,
	Wall,
	Letter (char),
}

impl Display for Tile {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Empty => write! (formatter, " "),
			Self::Passage => write! (formatter, "."),
			Self::Wall => write! (formatter, "#"),
			Self::Letter (ch) => write! (formatter, "{}", ch),
		}
	}
}

impl <'inp> FromParser <'inp> for Tile {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let val = match parser.peek ().ok_or_else (|| parser.err ()) ? {
			' ' => Self::Empty,
			'.' => Self::Passage,
			'#' => Self::Wall,
			ch @ 'A' ..= 'Z' => Self::Letter (ch),
			_ => return Err (parser.err ()),
		};
		parser.expect_next () ?;
		Ok (val)
	}
}

#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Portal {
	label: [char; 2],
	inner: bool,
}

impl Portal {

	#[ must_use ]
	pub fn new (label: [char; 2], inner: bool) -> Self {
		assert! (label.iter ().all (|& ch| ch.is_ascii_uppercase ()));
		Self { label, inner }
	}

	#[ must_use ]
	pub const fn label (self) -> [char; 2] {
		self.label
	}

	#[ must_use ]
	pub const fn inner (self) -> bool {
		self.inner
	}

	#[ must_use ]
	pub const fn partner (self) -> Self {
		Self { label: self.label, inner: ! self.inner }
	}

	pub const AA: Self = Self { label: [ 'A', 'A' ], inner: false };
	pub const ZZ: Self = Self { label: [ 'Z', 'Z' ], inner: false };

}

impl Debug for Portal {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"Portal (\"{ch_0}{ch_1}\", {dir})",
			ch_0 = self.label [0],
			ch_1 = self.label [1],
			dir = if self.inner { "inner" } else { "outer" })
	}
}
