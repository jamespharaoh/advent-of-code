use super::*;

use model::Step;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub tiles: Vec <InputTile>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { tiles, params } = [ params, @lines tiles ]
}

wrapper_deref_mut! {
	#[ derive (Clone, Debug) ]
	pub struct InputTile {
		pub steps: Vec <Step>,
	}
}

struct_parser_display! {
	InputTile { steps } = [ @collect steps ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_iters: u32 = ("NUM_ITERS=", 100, 1_u32 .. ),
	}
}
