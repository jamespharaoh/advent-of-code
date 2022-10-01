use super::*;

use model::Token;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub lines: Vec <InputLine>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { lines, params } = [ params, @lines lines ]
}

wrapper_deref_mut! {
	#[ derive (Clone, Debug) ]
	pub struct InputLine {
		pub tokens: Vec <Token>,
	}
}

struct_parser_display! {
	InputLine { tokens } = [ @collect tokens ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
