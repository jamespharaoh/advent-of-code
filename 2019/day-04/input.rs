use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub min: u32,
	pub max: u32,
	pub params: InputParams,
}

struct_parser_display! {
	Input { min, max, params } = [ params, min, "-", max ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
