use super::*;

use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub points: Vec <Pos>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { points, params } = [
		params,
		@lines points {
			Pos { x, y, z } = [ x, ",", y, ",", z ],
		},
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
