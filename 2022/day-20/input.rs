use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub nums: Vec <i32>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { nums, params } = [
		params,
		@lines nums,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
