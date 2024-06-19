use super::*;

use model::Grid;
use model::Step;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub grid: Grid,
	pub path: Vec <Step>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { grid, path, params } = [
		params,
		grid, "\n",
		"\n",
		@collect path,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
