use super::*;

pub type Coord = i16;
pub type Grid = GridBuf <Vec <Region>, Pos, 2>;
pub type Pos = aoc_pos::PosYX <Coord>;
pub type Val = u32;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Region {
		Rocky = ".",
		Wet = "=",
		Narrow = "|",
	}
}

impl Region {

	#[ must_use ]
	pub const fn can_equip (self, equip: Equip) -> bool {
		! matches! ((self, equip),
			(Self::Rocky, Equip::Neither) |
			(Self::Wet, Equip::Torch) |
			(Self::Narrow, Equip::Climbing))
	}

}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum Equip {
	Torch,
	Climbing,
	Neither,
}
