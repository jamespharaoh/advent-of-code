//! Data structures to model the puzzle input

use super::*;

use model::Room;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub rooms: Vec <Room <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { rooms, params } = [ params, @lines rooms ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
