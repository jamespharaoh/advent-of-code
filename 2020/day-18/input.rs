use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub exprs: Vec <InputExpr>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { exprs, params } = [ params, @lines exprs ]
}

wrapper_deref_mut! {
	#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct InputExpr {
		pub tokens: Vec <InputToken>,
	}
}

struct_parser_display! {
	InputExpr { tokens } = [ @collect tokens ]
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum InputToken {
	ParenOpen,
	ParenClose,
	Number (u16),
	Add,
	Mul,
}

enum_parser_display! {
	InputToken,
	ParenOpen = [ "(" ],
	ParenClose = [ ")" ],
	Number (val) = [ val ],
	Add = [ @skip " ", "+", @skip " " ],
	Mul = [ @skip " ", "*", @skip " " ],
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
