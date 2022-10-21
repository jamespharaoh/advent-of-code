use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub pub_key_0: u32,
	pub pub_key_1: u32,
	pub params: InputParams,
}

struct_parser_display! {
	Input { pub_key_0, pub_key_1, params } = [ params, pub_key_0, "\n", pub_key_1 ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_loops: u32 = ("MAX_LOOPS=", 2_000_000, 1 .. ),
	}
}
