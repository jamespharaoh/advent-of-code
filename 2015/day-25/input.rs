//! Data structures to model the puzzle input

use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub row: u64,
	pub col: u64,
	pub params: InputParams,
}

struct_parser_display! {
	Input { row, col, params } = [
		params,
		"To continue, please consult the code grid in the manual.  Enter the code at row ",
		row = (1 .. ), ", column ",
		col = (1 .. ), ".",
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
