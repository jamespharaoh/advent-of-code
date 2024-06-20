use super::*;

use model::Snafu;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub snafus: Vec <Snafu>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { snafus, params } = [
		params,
		@lines snafus,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
