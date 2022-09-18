use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub adapters: Vec <u16>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { adapters, params } = [ params, @lines adapters ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
