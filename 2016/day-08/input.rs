//! Data structures to model the puzzle input

use super::*;

use model::Coord;
use model::Step;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub steps: Vec <Step>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { steps, params } = [ params, @lines steps ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub width: Coord = ("WIDTH=", 50, 1_u32 .. ),
		pub height: Coord = ("HEIGHT=", 6, 1_u32 .. ),
	}
}
