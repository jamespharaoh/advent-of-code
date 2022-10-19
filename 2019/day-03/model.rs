use super::*;

pub type Dir = aoc_pos::Dir2d;
pub type Pos = aoc_pos::PosYX <Val>;
pub type Val = i16;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Step {
	pub dir: Dir,
	pub num: Val,
}

struct_parser_display! {
	Step { dir, num } = [
		dir {
			type = Dir;
			Dir::Up = [ "U" ],
			Dir::Down = [ "D" ],
			Dir::Left = [ "L" ],
			Dir::Right = [ "R" ],
		},
		num,
	]
}
