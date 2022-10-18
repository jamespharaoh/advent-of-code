use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub data: Vec <u8>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { data, params } = [ params, @delim " " data ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
