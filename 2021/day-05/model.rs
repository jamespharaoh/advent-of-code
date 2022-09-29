use super::*;

pub type Coord = i16;
pub type Grid = GridBuf <Vec <Val>, Pos, 2>;
pub type Pos = aoc_pos::PosYX <Coord>;
pub type Val = u16;

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
pub struct Vent {
	pub start: Pos,
	pub end: Pos,
}

struct_parser_display! {
	Vent {
		start: Pos { y: start_y, x: start_x },
		end: Pos { y: end_y, x: end_x },
	} = [
		start_x, ",", start_y, " -> ",
		end_x, ",", end_y,
	]
}

impl Vent {

	#[ inline ]
	#[ must_use ]
	pub const fn is_point (self) -> bool {
		self.start.y == self.end.y && self.start.x == self.end.x
	}

	#[ inline ]
	#[ must_use ]
	pub const fn is_straight (self) -> bool {
		(self.start.y == self.end.y) != (self.start.x == self.end.x)
	}

	#[ inline ]
	#[ must_use ]
	pub const fn is_diagonal (self) -> bool {
		self.start.y != self.end.y && self.start.x != self.end.x
			&& (self.end.y - self.start.y).unsigned_abs ()
				== (self.end.x - self.start.x).unsigned_abs ()
	}

	#[ inline ]
	#[ must_use ]
	pub const fn is_valid (self) -> bool {
		self.is_point () || self.is_straight () || self.is_diagonal ()
	}

}
