use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub histories: Vec <Vec <i64>>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { histories, params } = [
		params,
		@lines histories {
			type = Vec <i64>; history = [ @delim_some " " history ],
		},
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
