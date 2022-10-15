//! Data structures to model the puzzle input

use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub discs: Vec <Disc>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { discs, params } = [ params, @lines discs ]
}

#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
pub struct Disc {
	pub delay: u8,
	pub num_posns: u8,
	pub start_pos: u8,
}

struct_parser_display! {
	Disc { delay, num_posns, start_pos } = [
		"Disc #", delay = 0 ..= 100, " ",
		"has ", num_posns = 1 ..= 100, " positions; ",
		"at time=0, it is at position ", start_pos, ".",
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
