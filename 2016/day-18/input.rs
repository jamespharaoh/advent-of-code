//! Data structures to model the puzzle input

use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub first_row: Vec <Tile>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { first_row, params } = [ params, @collect first_row ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_rows_one: u32 = ("NUM_ROWS_ONE=", 40, 1 .. ),
		pub num_rows_two: u32 = ("NUM_ROWS_TWO=", 400_000, 1 .. ),
	}
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub enum Tile {
		Trap = [ "^" ],
		Safe = [ "." ],
	}
}
