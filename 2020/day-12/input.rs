use super::*;

use model::Step;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub steps: Vec <Step>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { steps, params } = [ params, @lines steps ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
