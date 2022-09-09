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
		pub max_ops_one: u32 = ("MAX_OPS_ONE=", 20, 1_u32 .. ),
		pub max_ops_two: u32 = ("MAX_OPS_TWO=", 100, 1_u32 .. ),
	}
}
