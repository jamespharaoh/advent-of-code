use super::*;

use model::Dir;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub moves: Vec <Dir>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { moves, params } = [
		params,
		@collect moves {
			type = Dir;
			Dir::North = [ "^" ],
			Dir::South = [ "v" ],
			Dir::East = [ ">" ],
			Dir::West = [ "<" ],
		},
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
