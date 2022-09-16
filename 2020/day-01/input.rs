use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub entries: Vec <u16>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { entries, params } = [
		params,
		@lines entries,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
