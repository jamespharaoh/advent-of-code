use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub lines: Vec <InpStr <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { lines, params } = [ params, @lines lines ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
