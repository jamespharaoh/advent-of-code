use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub start_nums: Vec <u32>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { start_nums, params } = [ params, @delim "," start_nums ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub ord_one: u32 = ("ORD_ONE=", 2020, 1 .. ),
		pub ord_two: u32 = ("ORD_TWO=", 30_000_000, 1 .. ),
	}
}
