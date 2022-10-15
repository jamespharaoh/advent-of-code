//! Data structures to model the puzzle input

use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub rules: Vec <Rule>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { rules, params } = [ params, @lines rules ]
}

#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
pub struct Rule {
	pub start: u32,
	pub end: u32,
}

struct_parser_display! {
	Rule { start, end } = [ start, "-", end ]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}
