use super::*;

use machine::Instr;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub instrs: Vec <Instr>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { instrs, params } = [ params, @lines instrs ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
