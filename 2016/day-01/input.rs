//! Data structures to model the puzzle input

use super::*;

use model::Coord;
use model::Turn;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub steps: Vec <(Turn, Coord)>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { steps, params } = [
		params,
		@delim ", " steps {
			(turn, dist) = [
				turn {
					Turn::Left = [ "L" ],
					Turn::Right = [ "R" ],
				},
				dist = (Coord::ONE .. ),
			],
		},
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
