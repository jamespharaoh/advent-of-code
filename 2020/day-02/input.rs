use super::*;

use model::PassPolicy;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub pass_policies: Vec <PassPolicy <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { pass_policies, params } = [ params, @lines pass_policies ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
