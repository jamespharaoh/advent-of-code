use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub data: Vec <u64>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { data, params } = [ params, @lines data ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
