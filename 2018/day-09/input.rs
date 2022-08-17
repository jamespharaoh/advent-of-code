use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub num_players: u32,
	pub last_marble: u32,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}

impl <'inp> FromParser <'inp> for Input {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser,
			params,
			num_players, " players; last marble is worth ", last_marble, " points");
		Ok (Self { num_players, last_marble, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter,
			"{num_players} players; last marble is worth {last_marble} points\n",
			num_players = self.num_players,
			last_marble = self.last_marble) ?;
		Ok (())
	}
}
