//! Data structures to model the puzzle input

use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub weights: Vec <u32>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { weights, params } = [ params, @lines weights ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
