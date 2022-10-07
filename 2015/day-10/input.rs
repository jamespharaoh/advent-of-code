use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub initial: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { initial, params } = [
		params,
		@str initial = ('1' ..= '3', 1 .. ),
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_iters_one: u32 = ("NUM_ITERS_ONE=", 40, (1_u32 .. )),
		pub num_iters_two: u32 = ("NUM_ITERS_TWO=", 50, (1_u32 .. )),
	}
}
