use super::*;

use cpu::Instr;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Input {
	pub instrs: Vec <Instr>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { instrs, params } = [ params, @lines instrs ]
}

input_params! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct InputParams {
	}
}
