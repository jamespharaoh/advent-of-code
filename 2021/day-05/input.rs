use super::*;

use model::Vent;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub vents: Vec <Vent>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { vents, params } = [ params, @lines vents ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
