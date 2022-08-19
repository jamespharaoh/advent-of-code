use super::*;

pub type Coord = i32;
pub type Pos = pos::PosYX <Coord>;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Point {
	pub pos: Pos,
	pub vel: Pos,
}

impl Point {

	#[ inline ]
	pub fn offset (self, offset: Coord) -> NumResult <Self> {
		Ok (Self {
			pos: Pos {
				y: Coord::add_2 (self.pos.y, Coord::mul_2 (self.vel.y, offset) ?) ?,
				x: Coord::add_2 (self.pos.x, Coord::mul_2 (self.vel.x, offset) ?) ?,
			},
			.. self
		})
	}

}

impl Display for Point {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"position=<{pos_x},{pos_y}>, velocity=<{vel_x},{vel_y}>",
			pos_x = self.pos.x,
			pos_y = self.pos.y,
			vel_x = self.vel.x,
			vel_y = self.vel.y,
		) ?;
		Ok (())
	}

}

impl <'inp> FromParser <'inp> for Point {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser,
			"position=<", @skip, pos_x, ",", @skip, pos_y, ">", @skip,
			"velocity=<", @skip, vel_x, ",", @skip, vel_y, ">");
		let pos = Pos { y: pos_y, x: pos_x };
		let vel = Pos { y: vel_y, x: vel_x };
		Ok (Self { pos, vel })
	}
}
