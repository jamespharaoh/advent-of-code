use super::*;

use model::Group;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub immune_system: Vec <Group>,
	pub infection: Vec <Group>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { immune_system, infection, params } = [
		params,
		"Immune System:\n",
		@lines immune_system, "\n",
		"\n",
		"Infection:\n",
		@lines infection,
	]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
		pub max_rounds: u32 = ("MAX_ROUNDS=", 6_000, 1 .. ),
		pub max_boost: u32 = ("MAX_BOOST=", 2_048, 1 .. ),
	}
}
