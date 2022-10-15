//! Data structures to model the puzzle input

use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub initial_state: Vec <Bit>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { initial_state, params } = [
		params,
		@collect_some_max 63 initial_state,
	]
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

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Bit {
		Zero = [ "0" ],
		One = [ "1" ],
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub disk_size_one: u32 = ("DISK_SIZE_ONE=", 272, 1 .. ),
		pub disk_size_two: u32 = ("DISK_SIZE_TWO=", 35_651_584, 1 .. ),
	}
}
