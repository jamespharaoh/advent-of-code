//! Data structures to model the puzzle input

use super::*;

use model::Button;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub buttons: Vec <Button>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { buttons, params } = [ params, @lines buttons ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
