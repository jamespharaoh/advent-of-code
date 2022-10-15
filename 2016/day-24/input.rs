use super::*;

use model::TilesGrid;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Input {
	pub tiles: TilesGrid,
	pub params: InputParams,
}

struct_parser_display! {
	Input { tiles, params } = [ params, tiles ]
}

input_params! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct InputParams {
	}
}
