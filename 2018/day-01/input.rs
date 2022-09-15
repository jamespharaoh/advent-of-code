use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub deltas: Vec <i32>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { deltas, params } = [ params, @lines deltas ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
