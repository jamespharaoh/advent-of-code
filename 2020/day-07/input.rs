use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub bags_contain: Vec <InputBagContains <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { bags_contain, params } = [ params, @lines bags_contain ]
}

#[ derive (Clone, Debug) ]
pub struct InputBagContains <'inp> {
	pub colour: InpStr <'inp>,
	pub contains: Vec <InputNumBags <'inp>>,
}

impl <'inp> Display for InputBagContains <'inp> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		let & Self { ref colour, ref contains } = self;
		display! (formatter, colour, " bags contain ");
		if self.contains.is_empty () {
			display! (formatter, "no other bags.");
		} else {
			display! (formatter, @delim ", " contains, ".");
		}
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for InputBagContains <'inp> {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, colour = parse_colour, " bags contain ");
		let contains = parser.any ()
			.of (|parser| { parse! (parser, @delim ", " contains, "."); Ok (contains) })
			.of (|parser| { parse! (parser, "no other bags."); Ok (Vec::new ()) })
			.done () ?;
		Ok (Self { colour, contains })
	}
}

#[ derive (Clone, Debug) ]
pub struct InputNumBags <'inp> {
	pub num: u32,
	pub colour: InpStr <'inp>,
}

impl <'inp> Display for InputNumBags <'inp> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"{num} {colour} {noun}",
			num = self.num,
			colour = self.colour,
			noun = if self.num != 1 { "bags" } else { "bag" })
	}
}

impl <'inp> FromParser <'inp> for InputNumBags <'inp> {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, num, " ", colour = parse_colour, " ");
		parser.any ()
			.of (|parser| { parser.expect ("bags") ?; Ok (()) })
			.of (|parser| { parser.expect ("bag") ?; Ok (()) })
			.done () ?;
		Ok (Self { num, colour })
	}
}

fn parse_colour <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	let num_chars =
		parser.peek_rest ().chars ().enumerate ()
			.filter (|& (_, ch)| ! ch.is_ascii_lowercase ())
			.nth (1)
			.map_or_else (
				|| parser.peek_rest ().chars ().count (),
				|(pos, _)| pos);
	parser.take_rest_while (|ch| ch.is_ascii_lowercase () || ch == ' ', ..= num_chars)
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_iters_one: u32 = ("MAX_ITERS_ONE=", 200, 1_u32 .. ),
		pub max_iters_two: u32 = ("MAX_ITERS_TWO=", 100, 1_u32 .. ),
	}
}
