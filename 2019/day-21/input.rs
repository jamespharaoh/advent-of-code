use super::*;

use model::Val;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub data: Vec <Val>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { data, params } = [ params, @delim "," data ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_ops: u32 = ("MAX_OPS=", 2_000_000, 1_u32 .. ),
	}
}
