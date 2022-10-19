use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub deps: Vec <(char, char)>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { deps, params } = [
		params,
		@lines deps { type = (char, char); (before, after) = [
			"Step ", before = 'A' ..= 'Z', " must be finished before ",
			"step ", after = 'A' ..= 'Z', " can begin.",
		] },
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_workers: u32 = ("NUM_WORKERS=", 5, 1_u32 ..= 10),
		pub extra_time: u32 = ("EXTRA_TIME=", 60, 0_u32 ..= 60),
	}
}
