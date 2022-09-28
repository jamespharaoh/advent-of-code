use super::*;

pub type Coord = i8;
pub type Cpu = intcode::Machine <Val>;
pub type Dir = pos::Dir2d;
pub type Grid = GridBuf <Vec <Tile>, Pos, 2>;
pub type Pos = pos::PosYX <Coord>;
pub type Turn = pos::Turn2d;
pub type Val = i32;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		Empty = ".",
		Scaffold = "#",
		RobotUp = "^",
		RobotDown = "v",
		RobotLeft = "<",
		RobotRight = ">",
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum Step {
	Left (u32),
	Right (u32),
}

impl Display for Step {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Left (dist) => write! (formatter, "L,{dist}"),
			Self::Right (dist) => write! (formatter, "R,{dist}"),
		}
	}
}
