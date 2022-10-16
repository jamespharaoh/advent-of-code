use super::*;

use model::Tramp;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Input {
	pub tramps: Vec <Tramp>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { tramps, params } = [ params, @lines tramps ]
}

input_params! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct InputParams {
	}
}
