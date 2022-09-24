use super::*;

#[ derive (Clone, Debug) ]
pub struct Food <'inp> {
	pub ingrs: Vec <InpStr <'inp>>,
	pub alrgns: Vec <InpStr <'inp>>,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Food <'inp> { ingrs, alrgns } = [
		@delim " " ingrs = parse_name, " (contains ",
		@delim ", " alrgns = parse_name, ")",
	]
}

fn parse_name <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	parser.take_rest_while (|ch| ch.is_ascii_lowercase (), 1 .. )
}
