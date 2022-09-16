use super::*;

pub type InputCoord = i8;
pub type InputGrid = grid::Grid <Vec <InputTile>, InputPos>;
pub type InputPos = pos::PosYX <InputCoord>;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub grid: InputGrid,
	pub params: InputParams,
}

struct_parser_display! {
	Input { grid, params } = [ params, grid ]
}

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum InputTile {
		#[ default ]
		Empty = ".",
		Bug = "#",
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
