use super::*;

use model::Val;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub target: Val,
	pub params: InputParams,
}

struct_parser_display! {
	Input { target, params } = [ params, target ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
