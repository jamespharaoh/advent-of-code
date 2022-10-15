//! Data structures to model the puzzle input

use super::*;

use model::Coord;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub seed: u32,
	pub params: InputParams,
}

struct_parser_display! {
	Input { seed, params } = [ params, seed ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub start_x: Coord = ("START_X=", 1, 1 .. ),
		pub start_y: Coord = ("START_Y=", 1, 1 .. ),
		pub end_x: Coord = ("END_X=", 31, 1 .. ),
		pub end_y: Coord = ("END_Y=", 39, 1 .. ),
		pub max_dist: u32 = ("MAX_DIST=", 500, 1 .. ),
		pub count_dist: u32 = ("COUNT_DIST=", 50, 1.. ),
	}
}
