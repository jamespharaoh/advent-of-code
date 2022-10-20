use super::*;

use model::Grid;
use model::PosXY;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub grid: Grid <PosXY, 2>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { grid, params } = [ params, grid ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub iters_one: u32 = ("ITERS_ONE=", 6, 1 .. ),
		pub iters_two: u32 = ("ITERS_TWO=", 6, 1 .. ),
	}
}
