use super::*;
use model::Grid;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub grid: Grid,
	pub params: InputParams,
}

struct_parser_display! (Input { grid, params } = [ params, grid ]);

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}
