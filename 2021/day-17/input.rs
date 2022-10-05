use super::*;

use model::Coord;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub target_x_start: Coord,
	pub target_x_end: Coord,
	pub target_y_start: Coord,
	pub target_y_end: Coord,
	pub params: InputParams,
}

struct_parser_display! {
	Input { target_x_start, target_x_end, target_y_start, target_y_end, params } = [
		params,
		"target area: ",
		"x=", target_x_start, "..", target_x_end, ", ",
		"y=", target_y_start, "..", target_y_end,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
