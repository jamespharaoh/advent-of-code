use super::*;

pub type Coord = i32;
pub type Dir = aoc_pos::DirGeo;
pub type Pos = aoc_pos::PosGeo <Coord>;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum Step {
	North (Coord),
	South (Coord),
	East (Coord),
	West (Coord),
	Left (Coord),
	Right (Coord),
	Forwards (Coord),
}

enum_parser_display! {
	Step,
	North (val) = [ "N", @confirm, val ],
	South (val) = [ "S", @confirm, val ],
	East (val) = [ "E", @confirm, val ],
	West (val) = [ "W", @confirm, val ],
	Left (val) = [ "L", @confirm, val ],
	Right (val) = [ "R", @confirm, val ],
	Forwards (val) = [ "F", @confirm, val ],
}
