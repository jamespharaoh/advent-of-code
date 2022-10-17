use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub start_a: u32,
	pub start_b: u32,
	pub params: InputParams,
}

struct_parser_display! {
	Input { start_a, start_b, params } = [
		params,
		"Generator A starts with ", start_a, "\n",
		"Generator B starts with ", start_b,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub reps_one: u32 = ("REPS_ONE=", 40_000_000, 0 .. ),
		pub reps_two: u32 = ("REPS_TWO=", 5_000_000, 0 .. ),
	}
}
