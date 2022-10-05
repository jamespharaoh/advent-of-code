use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub nums: Vec <Tokens>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { nums, params } = [ params, @lines nums ]
}

#[ derive (Clone, Debug) ]
pub struct Tokens {
	pub tokens: Vec <Token>,
}

struct_parser_display! {
	Tokens { tokens } = [ @collect tokens ]
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum Token {
	Open,
	Close,
	Comma,
	Value (u8),
}

enum_parser_display! {
	Token,
	Open = [ "[" ],
	Close = [ "]" ],
	Comma = [ "," ],
	Value (val) = [ val = 0 ..= 9 ],
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
