use super::*;
use cpu::Instr;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub instrs: Vec <Instr <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { instrs, params } = [ params, @lines instrs ]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}
