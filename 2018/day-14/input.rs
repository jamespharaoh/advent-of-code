use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub value: InpStr <'inp>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
		pub max_recipes: u32 = ("MAX_RECIPIES=", 50_000_000, 1_u32 .. ),
	}
}

impl <'inp> FromParser <'inp> for Input <'inp> {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		fn parse_value <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
			parser.take_rest_while (|ch| ch.is_ascii_digit (), 1 ..= 6)
		}
		parse! (parser, params, value = parse_value);
		if ! (1 ..= 8).contains (& value.len ()) { return Err (parser.err ()) }
		Ok (Self { value, params })
	}
}

impl <'inp> Display for Input <'inp> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "{}\n", self.value) ?;
		Ok (())
	}
}
