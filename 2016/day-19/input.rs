//! Data structures to model the puzzle input

use super::*;

#[ derive (Clone, Copy, Debug) ]
pub struct Input {
	pub num_elves: u32,
	pub params: InputParams,
}

struct_parser_display! {
	Input { num_elves, params } = [ params, num_elves = 2 .. ]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}
