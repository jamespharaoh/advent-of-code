use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub data: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { data, params } = [
		params,
		@str data = (|ch| { ch.is_ascii_lowercase () }, 1 .. ),
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
