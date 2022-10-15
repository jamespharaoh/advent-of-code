use super::*;

pub type Coord = u32;
pub type Pos = pos::PosRowCol <Coord>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Step {
		Rect { width: Coord, height: Coord } = [ "rect ", width, "x", height ],
		RotateRow { row: Coord, dist: Coord } = [ "rotate row y=", row, " by ", dist ],
		RotateCol { col: Coord, dist: Coord } = [ "rotate column x=", col, " by ", dist ],
	}
}
