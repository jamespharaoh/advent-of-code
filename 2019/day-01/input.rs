use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub module_masses: Vec <u32>,
	pub params: InputParams,
}

struct_parser_display! {
	Input {
		module_masses,
		params,
	} = [
		params,
		@lines module_masses,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
