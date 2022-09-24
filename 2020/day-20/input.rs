use super::*;

use model::Grid;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub tiles: Vec <InputTile>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { tiles, params } = [ params, @delim "\n\n" tiles ]
}

#[ derive (Clone, Debug) ]
pub struct InputTile  {
	pub id: u16,
	pub grid: Grid,
}

struct_parser_display! {
	InputTile { id, grid } = [ "Tile ", id, ":\n", grid ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
