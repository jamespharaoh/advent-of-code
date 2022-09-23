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
		pub max_step_ops: u32 = ("MAX_STEP_OPS=", 500, 1_u32 .. ),
		pub max_steps: u32 = ("MAX_STEPS=", 20_000, 1_u32 .. ),
	}
}
