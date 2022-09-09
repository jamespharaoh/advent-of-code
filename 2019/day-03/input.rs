use super::*;

use model::Step;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub wire_0: Vec <Step>,
	pub wire_1: Vec <Step>,
	pub params: InputParams,
}

struct_parser_display! {
	Input {
		wire_0,
		wire_1,
		params,
	} = [
		params,
		@delim "," wire_0, "\n",
		@delim "," wire_1,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
