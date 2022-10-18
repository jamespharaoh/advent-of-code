use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub polymer: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { polymer, params } = [
		params,
		@str polymer = (|ch| { ch.is_ascii_alphabetic () }, 1 .. ),
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
