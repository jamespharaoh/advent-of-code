use super::*;

use model::AuntSue;

#[ derive (Clone, Debug) ]
pub struct Input  {
	pub sues: Vec <AuntSue>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { sues, params } = [ params, @lines sues ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
