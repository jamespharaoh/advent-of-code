use super::*;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Input {
	pub digits: Vec <u8>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { digits, params } = [ params, @collect digits = digit_parse ]
}

fn digit_parse (parser: & mut Parser) -> ParseResult <u8> {
	if ! matches! (parser.peek (), Some ('0' ..= '9')) { return Err (parser.err ()) }
	Ok (parser.next ().unwrap ().to_digit (10).unwrap ().pan_u8 ())
}

input_params! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct InputParams {
	}
}
