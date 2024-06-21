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
	}
}
