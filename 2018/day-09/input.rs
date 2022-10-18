use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub num_players: u32,
	pub last_marble: u32,
	pub params: InputParams,
}

struct_parser_display! {
	Input { num_players, last_marble, params } = [
		params,
		num_players, " players; ",
		"last marble is worth ", last_marble, " points",
	]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}
