use super::*;

pub type Coord = i64;
pub type Axis = pos::AxisXY;
pub type Pos = pos::PosYX <Coord>;

#[ derive (Clone, Debug) ]
pub struct Fold {
	pub axis: Axis,
	pub val: Coord,
}

struct_parser_display! {
	Fold { axis, val } = [
		"fold along ",
		axis { Axis::X = [ "x" ], Axis::Y = [ "y" ] },
		"=",
		val,
	]
}
