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
		pub step_max_ops: u32 = ("STEP_MAX_OPS=", 200, 1_u32 .. ),
		pub core_max_steps: u32 = ("CORE_MAX_STEPS=", 15_000, 1_u32 .. ),
		pub game_max_steps: u32 = ("GAME_MAX_STEPS=", 15_000, 1_u32 .. ),
		pub size_max_steps: u32 = ("SIZE_MAX_STEPS=", 1_000, 1_u32 .. ),
	}
}
