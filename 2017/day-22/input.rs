use super::*;

use model::Grid;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub nodes: Grid,
	pub params: InputParams,
}

struct_parser_display! {
	Input { nodes, params } = [ params, nodes ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub iters_one: u32 = ("ITERS_ONE=", 10_000, 1_u32 .. ),
		pub iters_two: u32 = ("ITERS_TWO=", 10_000_000, 1_u32 .. ),
	}
}
