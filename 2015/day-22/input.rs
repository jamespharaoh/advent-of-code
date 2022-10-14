//! Data structures to model the puzzle input

use super::*;

use model::Boss;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub boss: Boss,
	pub params: InputParams,
}

struct_parser_display! {
	Input { boss, params } = [ params, boss ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
