use super::*;

use model::Grid;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub grid: Grid,
	pub params: InputParams,
}

struct_parser_display! {
	Input { grid, params } = [ params, grid ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_iters: u32 = ("MAX_ITERS=", 50_000, 1_u32 .. ),
		pub max_path_starts: u32 = ("MAX_PATH_STARTS=", 40, 1_u32 .. ),
	}
}
