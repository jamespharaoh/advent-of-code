use super::*;

pub type Coord = u16;
pub type Grid = grid::Grid <Vec <Tile>, Pos>;
pub type Pos = pos::PosYX <Coord>;

parse_display_enum! {

	#[ derive (Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		DrySand = ".",
		WetSand = "|",
		Water = "~",
		Clay = "#",
	}

}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum ClayRange {
	Horiz { y: Coord, x_start: Coord, x_end: Coord },
	Vert { x: Coord, y_start: Coord, y_end: Coord },
}

enum_parser_display! (
	ClayRange,
	Horiz { y, x_start, x_end } = [ "y=", y, ", x=", x_start, "..", x_end ],
	Vert { x, y_start, y_end } = [ "x=", x, ", y=", y_start, "..", y_end ],
);

impl ClayRange {

	#[ inline ]
	#[ must_use ]
	pub const fn ranges (self) -> (RangeInclusive <Coord>, RangeInclusive <Coord>) {
		match self {
			Self::Horiz { y, x_start, x_end } => (y ..= y, x_start ..= x_end),
			Self::Vert { x, y_start, y_end } => (y_start ..= y_end, x ..= x),
		}
	}

	#[ inline ]
	#[ must_use ]
	pub const fn y (self) -> RangeInclusive <Coord> {
		self.ranges ().0
	}

	#[ inline ]
	#[ must_use ]
	pub const fn x (self) -> RangeInclusive <Coord> {
		self.ranges ().1
	}

	#[ inline ]
	#[ must_use ]
	pub fn valid (self) -> bool {
		! (self.y ().is_empty () || self.x ().is_empty ())
	}

}
