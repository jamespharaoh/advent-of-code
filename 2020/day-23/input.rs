use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub start: InpStr <'inp>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { start, params } = [
		params,
		@str start = ('1' ..= '9', 9 ..= 9),
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub iters_one: u32 = ("ITERS_ONE=", 100, 1_u32 .. ),
		pub iters_two: u32 = ("ITERS_TWO=", 10_000_000, 1_u32 .. ),
		pub deck_size_two: u32 = ("DECK_SIZE_TWO=", 1_000_000, 1_u32 .. ),
	}
}
