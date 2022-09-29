use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub report: Vec <u16>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { report, params } = [ params, @lines report ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
