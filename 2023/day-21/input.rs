use super::*;

use model::Grid;
use model::Tile;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub grid: Grid <Tile>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { grid, params } = [ params, grid ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_steps_one: u32 = ("NUM_STEPS_ONE=", 64, 1 .. ),
		pub num_steps_two: u32 = ("NUM_STEPS_TWO=", 26_501_365, 1 .. ),
		pub test: u8 = ("TEST=", 0, .. ),
	}
}
