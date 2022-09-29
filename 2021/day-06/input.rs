use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub fish: Vec <u8>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { fish, params } = [ params, @delim "," fish = (0 ..= 8) ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
