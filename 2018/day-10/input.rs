use super::*;
use model::Point;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub points: Vec <Point>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { points, params } = [ params, @lines points ]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}
