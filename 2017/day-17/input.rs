use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub advance: u32,
	pub params: InputParams,
}

struct_parser_display! {
	Input { advance, params } = [ params, advance ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
