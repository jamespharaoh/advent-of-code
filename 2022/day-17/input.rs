use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub pushes: Vec <Push>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { pushes, params } = [ params, @collect pushes ]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug) ]
	pub enum Push {
		Left = [ "<" ],
		Right = [ ">" ],
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_iters: u64 = ("MAX_ITERS=", 5_000, 1 .. ),
		pub num_drops_one: u64 = ("NUM_DROPS_ONE=", 2022, 1 .. ),
		pub num_drops_two: u64 = ("NUM_DROPS_TWO=", 1_000_000_000_000, 1 .. ),
	}
}
