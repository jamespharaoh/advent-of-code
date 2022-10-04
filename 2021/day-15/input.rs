use super::*;

use model::Risks;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub risks: Risks,
	pub params: InputParams,
}

struct_parser_display! {
	Input { risks, params } = [ params, risks = risks_parse ]
}

fn risks_parse (parser: & mut Parser) -> ParseResult <Risks> {
	Risks::parse_with_fn (parser, || 1, |parser| {
		if ! matches! (parser.peek (), Some ('1' ..= '9')) { return Err (parser.err ()) }
		Ok (parser.next ().unwrap ().to_digit (10).unwrap ().pan_u8 ())
	})
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
