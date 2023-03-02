use super::*;

use model::Coord;
use model::Dir;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub steps: Vec <Step>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { steps, params } = [ params, @lines steps ]
}

#[ derive (Clone, Debug) ]
pub struct Step {
	pub dir: Dir,
	pub num: Coord,
}

struct_parser_display! {
	Step { dir, num } = [
		dir {
			type = Dir;
			Dir::Up = [ "U" ],
			Dir::Down = [ "D" ],
			Dir::Left = [ "L" ],
			Dir::Right = [ "R" ],
		},
		" ",
		num = 1 ..= 25,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
