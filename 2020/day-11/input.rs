use super::*;

use model::Tiles;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub tiles: Tiles,
	pub params: InputParams,
}

struct_parser_display! {
	Input { tiles, params } = [ params, tiles ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_iters: u32 = ("MAX_ITERS=", 100, 1_u32 .. ),
	}
}
