use super::*;

use model::Grid;

#[ derive (Clone, Debug) ]
pub struct Input  {
	pub grid: Grid,
	pub params: InputParams,
}

struct_parser_display! {
	Input { grid, params } = [ params, grid ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_steps: u32 = ("NUM_STEPS=", 100, 1_u32 .. ),
	}
}
