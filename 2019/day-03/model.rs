use super::*;

pub type Dir = aoc_pos::Dir2d;
pub type Pos = aoc_pos::PosYX <Val>;
pub type Val = i16;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Step {
	pub dir: Dir,
	pub num: Val,
}

impl Display for Step {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"{dir}{num}",
			dir = match self.dir {
				Dir::Up => 'U',
				Dir::Down => 'D',
				Dir::Left => 'L',
				Dir::Right => 'R',
			},
			num = self.num)
	}
}

impl <'inp> FromParser <'inp> for Step {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, dir_ch: char);
		let dir = match dir_ch {
			'U' => Dir::Up,
			'D' => Dir::Down,
			'L' => Dir::Left,
			'R' => Dir::Right,
			_ => return Err (parser.err ()),
		};
		parse! (parser, num);
		Ok (Self { dir, num })
	}
}
