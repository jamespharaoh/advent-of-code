use super::*;

#[ derive (Clone, Debug, Eq, Ord, PartialEq, PartialOrd) ]
pub struct Orbit <'inp> {
	pub base: InpStr <'inp>,
	pub satl: InpStr <'inp>,
}

struct_parser! {
	input_lifetime = 'inp;
	Orbit <'inp> { base, satl } = [ base = parse_object, ")", satl = parse_object ]
}

struct_display! {
	input_lifetime = 'inp;
	Orbit <'inp> { base, satl } = [ base, ")", satl ]
}

fn parse_object <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	parser.take_rest_while (|ch| ch.is_ascii_alphanumeric (), .. )
}
