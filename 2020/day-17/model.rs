use super::*;

pub type Coord = i8;
pub type PosXY = aoc_pos::PosXY <Coord>;
pub type PosXYZ = aoc_pos::PosXYZ <Coord>;
pub type PosXYZW = aoc_pos::PosXYZW <Coord>;
pub type Grid <Pos, const DIMS: usize> = aoc_grid::Grid <Vec <Tile>, Pos, DIMS>;
pub type GridOffset <const DIMS: usize> = aoc_grid::GridOffset <DIMS>;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		Inactive = ".",
		Active = "#",
	}
}

pub trait GenPos <const DIMS: usize>:
	aoc_pos::GenPos <DIMS, Val = Coord> +
	aoc_grid::GridPos <DIMS> {
}

impl <Pos, const DIMS: usize> GenPos <DIMS> for Pos where Pos:
	aoc_pos::GenPos <DIMS, Val = Coord> +
	aoc_grid::GridPos <DIMS> {
}
