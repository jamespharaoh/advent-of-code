use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub player_1: u8,
	pub player_2: u8,
	pub params: InputParams,
}

struct_parser_display! {
	Input { player_1, player_2, params } = [
		params,
		"Player 1 starting position: ", player_1 = 1 ..= 10, "\n",
		"Player 2 starting position: ", player_2 = 1 ..= 10,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
