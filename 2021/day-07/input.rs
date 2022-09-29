use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub crabs: Vec <u16>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { crabs, params } = [ params, @delim "," crabs = (0 ..= 9_999) ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
