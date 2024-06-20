use super::*;

use model::Grid;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub grid: Grid,
	pub params: InputParams,
}

struct_parser_display! {
	Input { grid, params } = [
		params,
		grid,
	]
}

enum_decl_parser_display! {

	#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
	pub enum Tile {
		#[ default ]
		Clear = [ "." ],
		Wall = [ "#" ],
		BlizzardLeft = [ "<" ],
		BlizzardRight = [ ">" ],
		BlizzardUp = [ "^" ],
		BlizzardDown = [ "v" ],
	}

}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
