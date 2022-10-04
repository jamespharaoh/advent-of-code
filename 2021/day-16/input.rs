use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub data: Vec <u8>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { data, params } = [ params, @collect data = (hex_byte_parse, hex_byte_display) ]
}

fn hex_byte_parse (parser: & mut Parser) -> ParseResult <u8> {
	let rest = parser.take_rest_while (|ch| ch.is_ascii_hexdigit (), 2 ..= 2) ?;
	Ok (u8::from_str_radix (& rest, 16).unwrap ())
}

fn hex_byte_display (byte: & u8, formatter: & mut fmt::Formatter) -> fmt::Result {
	write! (formatter, "{byte:02X}")
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
