use super::*;

use model::Card;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub player_1: Vec <Card>,
	pub player_2: Vec <Card>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { player_1, player_2, params } = [
		params,
		"Player 1:\n", @lines player_1, "\n",
		"\n",
		"Player 2:\n", @lines player_2,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_games: u32 = ("MAX_GAMES=", 50_000, 1 .. ),
		pub max_rounds: u32 = ("MAX_ROUNDS=", 2_000_000, 1 .. ),
	}
}
