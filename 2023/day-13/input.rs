use super::*;

use model::Grid;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub grids: Vec <Grid>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { grids, params } = [ params, @delim "\n\n" grids ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
