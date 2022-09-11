use super::*;
use model::Reaction;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub reactions: Vec <Reaction <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { reactions, params } = [ params, @lines reactions ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_ore: u64 = ("NUM_ORE=", 1_000_000_000_000, 1_u64 .. ),
	}
}
