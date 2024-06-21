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
		pub expand_one: i64 = ("EXPAND_ONE=", 1, 1 ..= 1_000_000),
		pub expand_two: i64 = ("EXPAND_TWO=", 999_999, 1 ..= 1_000_000_000),
	}
}
