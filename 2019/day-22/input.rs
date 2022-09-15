use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub shuffles: Vec <Shuffle>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { shuffles, params } = [ params, @lines shuffles ]
}

#[ derive (Clone, Copy, Debug) ]
pub enum Shuffle {
	DealIntoNewStack,
	Cut (i32),
	DealWithIncrement (u32),
}

enum_parser_display! {
	Shuffle,
	DealIntoNewStack = [ "deal into new stack" ],
	Cut (num) = [ "cut ", num ],
	DealWithIncrement (num) = [ "deal with increment ", num ],
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub deck_size_one: u64 = ("DECK_SIZE_ONE=", 10_007, 1_u64 .. ),
		pub deck_size_two: u64 = ("DECK_SIZE_TWO=", 119_315_717_514_047, 1_u64 .. ),
		pub repeat_two: u64 = ("REPEAT_TWO=", 101_741_582_076_661, 1_u64 .. ),
		pub init_one: u64 = ("INIT_ONE=", 2019, 1_u64 .. ),
		pub init_two: u64 = ("INIT_TWO=", 2020, 1_u64 .. ),
	}
}
