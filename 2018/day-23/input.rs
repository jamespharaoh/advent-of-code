use super::*;
use model::Nanobot;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub nanobots: Vec <Nanobot>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { nanobots, params } = [ params, @lines nanobots ]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
		pub max_iters: u32 = ("MAX_ITERS=", 5_000, 1 .. ),
	}
}
