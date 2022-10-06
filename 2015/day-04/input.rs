use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub seed: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { seed, params } = [
		params,
		@str seed = (|ch| { ch.is_ascii_lowercase () }, 1 .. 100),
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_zeros_one: u32 = ("NUM_ZEROS_ONE=", 5, 1_u32 .. ),
		pub num_zeros_two: u32 = ("NUM_ZEROS_TWO=", 6, 1_u32 .. ),
		pub max_threads: u32 = ("MAX_THREADS=", u32::MAX, 1_u32 .. ),
	}
}
