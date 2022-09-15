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
		pub max_iters_one: u32 = ("MAX_ITERS_ONE=", 100, 1_u32 .. ),
		pub max_iters_two: u32 = ("MAX_ITERS_TWO=", 1000, 1_u32 .. ),
		pub max_ops: u32 = ("MAX_OPS=", 1000, 1_u32 .. ),
	}
}
