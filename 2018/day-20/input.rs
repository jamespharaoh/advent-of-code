use super::*;

use model::RouteRegex;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub regex: RouteRegex,
	pub params: InputParams,
}

struct_parser_display! {
	Input { regex, params } = [ params, regex ]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
		pub dist_two: u32 = ("DIST_TWO=", 1000, 1_u32 .. ),
	}
}
