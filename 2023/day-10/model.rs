use super::*;

pub type Coord = i16;
pub type Dir = aoc_pos::DirGeo;
pub type Grid = aoc_grid::GridBuf <Vec <Tile>, Pos, 2>;
pub type Pos = aoc_pos::PosGeo <Coord>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default) ]
	pub enum Tile {
		Vertical = [ "|" ],
		Horizontal = [ "-" ],
		NorthEast = [ "L" ],
		NorthWest = [ "J" ],
		SouthWest = [ "7" ],
		SouthEast = [ "F" ],
		#[ default ]
		Ground = [ "." ],
		Start = [ "S" ],
	}
}

impl Tile {
	pub fn is_start (self) -> bool {
		matches! (self, Self::Start)
	}
	pub fn is_pipe (self) -> bool {
		match self {
			Self::Vertical => true,
			Self::Horizontal => true,
			Self::NorthEast => true,
			Self::NorthWest => true,
			Self::SouthWest => true,
			Self::SouthEast => true,
			Self::Ground => false,
			Self::Start => true,
		}
	}
	pub fn dirs (self) -> [Dir; 2] {
		match self {
			Self::Vertical => [ Dir::North, Dir::South ],
			Self::Horizontal => [ Dir::West, Dir::East ],
			Self::NorthEast => [ Dir::North, Dir::East ],
			Self::NorthWest => [ Dir::North, Dir::West ],
			Self::SouthWest => [ Dir::South, Dir::West ],
			Self::SouthEast => [ Dir::South, Dir::East ],
			Self::Ground | Self::Start => panic! (),
		}
	}
	pub fn follow (self, dir: Dir) -> Dir {
		let dirs = self.dirs ();
		if dir.around () == dirs [0] { return dirs [1] }
		if dir.around () == dirs [1] { return dirs [0] }
		panic! ();
	}
	pub fn for_dirs (dirs: [Dir; 2]) -> Self {
		match dirs {
			[ Dir::North, Dir::South ] | [ Dir::South, Dir::North ] => Self::Vertical,
			[ Dir::East, Dir::West ] | [ Dir::West, Dir::East ] => Self::Horizontal,
			[ Dir::North, Dir::East ] | [ Dir::East, Dir::North ] => Self::NorthEast,
			[ Dir::North, Dir::West ] | [ Dir::West, Dir::North ] => Self::NorthWest,
			[ Dir::South, Dir::East ] | [ Dir::East, Dir::South ] => Self::SouthEast,
			[ Dir::South, Dir::West ] | [ Dir::West, Dir::South ] => Self::SouthWest,
			_ => panic! (),
		}
	}
}
