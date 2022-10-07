use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub initial: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { initial, params } = [
		params,
		@str initial = ('a' ..= 'z', 8 ..= 8),
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
