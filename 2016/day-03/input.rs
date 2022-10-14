//! Data structures to model the puzzle input

use super::*;

use model::Triangle;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub triangles: Vec <Triangle>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { triangles, params } = [
		params,
		@lines triangles {
			(a, b, c) = [ @skip "", a, @skip " ", b, @skip " ", c ],
		}
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
