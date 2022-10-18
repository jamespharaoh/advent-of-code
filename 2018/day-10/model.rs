use super::*;

pub type Coord = i32;
pub type Pos = pos::PosYX <Coord>;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Point {
	pub pos: Pos,
	pub vel: Pos,
}

struct_parser_display! {
	Point { pos: Pos { y: pos_y, x: pos_x }, vel: Pos { y: vel_y, x: vel_x } } = [
		"position=<", @skip "", pos_x, ",", @skip " ", pos_y, "> ",
		"velocity=<", @skip "", vel_x, ",", @skip " ", vel_y, ">",
	]
}

impl Point {

	#[ inline ]
	pub fn offset (self, offset: Coord) -> NumResult <Self> {
		Ok (Self {
			pos: Pos {
				y: chk! (self.pos.y + self.vel.y * offset) ?,
				x: chk! (self.pos.x + self.vel.x * offset) ?,
			},
			.. self
		})
	}

}
