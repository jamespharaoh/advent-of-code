use super::*;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Input <'inp> {
	pub passphrases: Vec <InputLine <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { passphrases, params } = [ params, @lines passphrases ]
}

wrapper_deref_mut! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct InputLine <'inp> {
		pub passphrases: Vec <InpStr <'inp>>,
	}
}

struct_parser_display! {
	input_lifetime = 'inp;
	InputLine <'inp> { passphrases } = [
		@delim " " passphrases = passphrase_parse,
	]
}

fn passphrase_parse <'inp> (parser: & mut Parser <'inp>) -> ParseResult <InpStr <'inp>> {
	parser.take_rest_while (|ch| ch.is_ascii_lowercase (), 1 ..= 16)
}

input_params! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct InputParams {
	}
}
