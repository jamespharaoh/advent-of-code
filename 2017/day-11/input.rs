use super::*;

use model::VHexDir;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub steps: Vec <VHexDir>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { steps, params } = [ params, @delim "," steps ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
