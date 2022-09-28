use super::*;
use model::ClayRange;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub clay_ranges: Vec <ClayRange>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { clay_ranges, params } = [ params, @lines clay_ranges ]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}
