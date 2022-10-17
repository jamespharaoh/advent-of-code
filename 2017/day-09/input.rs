use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub input: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { input, params } = [ params, input ]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}
