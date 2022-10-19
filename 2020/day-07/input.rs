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

struct_parser_display! {
	input_lifetime = 'inp;
	InputBagContains <'inp> { colour, contains } = [
		colour = parse_colour, " bags contain ", contains {
			type = Vec <InputNumBags>;
			contains if (contains.is_empty ()) = [
				"no other bags",
				@parse contains { Vec::new () },
			],
			contains = [ @delim ", " contains ],
		}, ".",
	]
}

#[ derive (Clone, Debug) ]
pub struct InputNumBags <'inp> {
	pub num: u32,
	pub colour: InpStr <'inp>,
}

struct_parser_display! {
	input_lifetime = 'inp;
	InputNumBags <'inp> { num, colour } = [
		(num, colour) {
			type = (& u32, & InpStr);
			(num, colour) if (* num != 1) = [ num, " ", colour = parse_colour, " bags" ],
			(num, colour) = [ num, " ", colour = parse_colour, " bag" ],
		},
	]
}

fn parse_colour <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	let num_chars =
		parser.peek_rest ().chars ().enumerate ()
			.filter (|& (_, ch)| ! ch.is_ascii_lowercase ())
			.nth (1)
			.map_or_else (
				|| parser.peek_rest ().chars ().count (),
				|(pos, _)| pos)
			.pan_u32 ();
	parser.take_rest_while (|ch| ch.is_ascii_lowercase () || ch == ' ', ..= num_chars)
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_iters_one: u32 = ("MAX_ITERS_ONE=", 200, 1 .. ),
		pub max_iters_two: u32 = ("MAX_ITERS_TWO=", 100, 1 .. ),
	}
}
