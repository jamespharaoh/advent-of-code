use super::*;

use model::Command;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub commands: Vec <Command>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { commands, params } = [ params, @lines commands ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
