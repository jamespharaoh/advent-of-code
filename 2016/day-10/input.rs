//! Data structures to model the puzzle input

use super::*;

use model::Step;
use model::Val;

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
		pub low: Val = ("LOW=", 17, .. ),
		pub high: Val = ("HIGH=", 61, .. ),
	}
}
