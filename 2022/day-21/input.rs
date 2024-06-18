use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub monkeys: Vec <Monkey <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	Input <'inp> { monkeys, params } = [
		params,
		@lines monkeys,
	]
}

#[ derive (Clone, Debug) ]
pub struct Monkey <'inp> {
	pub id: InpStr <'inp>,
	pub op: MonkeyOp <'inp>,
}

struct_parser_display! {
	Monkey <'inp> { id, op } = [
		id = parse_id, ": ", op,
	]
}

enum_decl_parser_display! {
	input_lifetime = 'inp;
	#[ derive (Clone, Debug) ]
	pub enum MonkeyOp <'inp> {
		Number (val: u32) = [ val ],
		Add (left: InpStr <'inp>, right: InpStr <'inp>) = [ left = parse_id, " + ", right = parse_id ],
		Sub (left: InpStr <'inp>, right: InpStr <'inp>) = [ left = parse_id, " - ", right = parse_id ],
		Mul (left: InpStr <'inp>, right: InpStr <'inp>) = [ left = parse_id, " * ", right = parse_id ],
		Div (left: InpStr <'inp>, right: InpStr <'inp>) = [ left = parse_id, " / ", right = parse_id ],
	}
}

fn parse_id <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	parser.take_rest_while (|ch| ch.is_ascii_lowercase () || ch == ' ', ..= 4)
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
