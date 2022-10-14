//! Data structures to model the puzzle input

use super::*;

use model::Stats;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub stats: Stats,
	pub params: InputParams,
}

struct_parser_display! {
	Input { stats, params } = [ params, stats ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
