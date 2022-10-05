use super::*;

use model::Algorithm;
use model::Pixels;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub algorithm: Algorithm,
	pub pixels: Pixels,
	pub params: InputParams,
}

struct_parser_display! {
	Input { algorithm, pixels, params } = [
		params,
		@array algorithm, "\n",
		"\n",
		pixels,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_times_one: u32 = ("NUM_TIMES_ONE", 2, 1_u32 .. ),
		pub num_times_two: u32 = ("NUM_TIMES_TWO", 50, 1_u32 .. ),
	}
}
