use super::*;

pub type Cpu = intcode::Machine <Val>;
pub type RcStr = Rc <str>;
pub type RunResult = intcode::RunResult <Val>;
pub type Val = i64;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
	pub enum Door {
		North = "north",
		South = "south",
		East = "east",
		West = "west",
	}
}

impl Door {

	#[ must_use ]
	pub const fn rev (self) -> Self {
		match self {
			Self::North => Self::South,
			Self::South => Self::North,
			Self::East => Self::West,
			Self::West => Self::East,
		}
	}

}
