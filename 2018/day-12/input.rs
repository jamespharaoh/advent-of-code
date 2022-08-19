use super::*;
use model::Pot;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub start: Vec <Pot>,
	pub rules: Vec <InputRule>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
		pub check_rules: bool = ("CHECK_RULES=", true, (false ..= true)),
	}
}

impl <'inp> FromParser <'inp> for Input {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, params, "initial state: ", @collect start, "\n\n", @lines rules);
		Ok (Self { start, rules, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "initial state: ") ?;
		for pot in self.start.iter () { Display::fmt (pot, formatter) ?; }
		write! (formatter, "\n\n") ?;
		for rule in self.rules.iter () { write! (formatter, "{}\n", rule) ?; }
		Ok (())
	}
}

#[ derive (Clone, Copy, Debug) ]
pub struct InputRule {
	pub from: [Pot; 5],
	pub to: Pot,
}

impl Display for InputRule {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"{}{}{}{}{} => {}",
			self.from [0], self.from [1], self.from [2], self.from [3], self.from [4],
			self.to,
		) ?;
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for InputRule {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, from_0, from_1, from_2, from_3, from_4, " => ", to);
		let from = [from_0, from_1, from_2, from_3, from_4];
		Ok (Self { from, to })
	}
}
