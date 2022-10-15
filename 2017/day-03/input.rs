use super::*;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Input {
	pub target: u32,
	pub params: InputParams,
}

struct_parser_display! {
	Input { target, params } = [ params, target ]
}

input_params! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct InputParams {
	}
}
