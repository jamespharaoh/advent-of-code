use super::*;

use model::Coord;
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
		Empty = [ "." ],
		Elf = [ "#" ],
	}

}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_size: Coord = ("MAX_SIZE=", 100, 1 .. ),
		pub max_rounds: u64 = ("MAX_ROUNDS=", 1000, 1 .. ),
	}
}
