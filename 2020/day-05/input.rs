use super::*;

use model::Seat;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub seats: Vec <Seat>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { seats, params } = [ params, @lines seats ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
