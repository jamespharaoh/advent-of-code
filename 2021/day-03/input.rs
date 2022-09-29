use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub readings: Vec <u16>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { readings, params } = [
		params,
		@lines readings = (reading_parse, reading_display),
	 ]
}

fn reading_parse (parser: & mut Parser) -> ParseResult <u16> {
	let rest = parser.take_rest_while (|ch| ('0' ..= '1').contains (& ch), 1 .. 16) ?;
	Ok (u16::from_str_radix (& rest, 2).unwrap ())
}

fn reading_display (reading: & u16, formatter: & mut fmt::Formatter) -> fmt::Result {
	write! (formatter, "{reading:012b}")
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
