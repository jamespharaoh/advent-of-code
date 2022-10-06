use super::*;

use model::Dir;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub dirs: Vec <Dir>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { dirs, params } = [ params, @collect dirs ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
