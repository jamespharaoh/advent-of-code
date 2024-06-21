use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub steps: Vec <Step <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { steps, params } = [ params, @delim "," steps ]
}

enum_decl_parser_display! {
	input_lifetime = 'inp;
	#[ derive (Clone, Debug) ]
	pub enum Step <'inp> {
		Insert (name: InpStr <'inp>, lens: u8) = [ name = parse_name, "=", lens ],
		Remove (name: InpStr <'inp>) = [ name = parse_name, "-" ],
	}
}

fn parse_name <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	parser.take_rest_while (|ch| ch.is_ascii_lowercase (), 1 ..= 6)
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
