use super::*;

#[ derive (Clone, Debug) ]
pub struct Input  {
	pub sizes: Vec <u32>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { sizes, params } = [ params, @lines sizes ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub target: u32 = ("TARGET=", 150, 1_u32 .. ),
	}
}
