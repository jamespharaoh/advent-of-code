use super::*;

pub type Coord = i8;
pub type Dir = pos::Dir2d;
pub type Pos = pos::PosRowCol <Coord>;

#[ derive (Clone, Debug) ]
pub struct Button {
	pub steps: Vec <Dir>,
}

struct_parser_display! {
	Button { steps } = [
		@collect steps {
			type = Dir;
			Dir::Up = [ "U" ],
			Dir::Down = [ "D" ],
			Dir::Left = [ "L" ],
			Dir::Right = [ "R" ],
		},
	]
}
