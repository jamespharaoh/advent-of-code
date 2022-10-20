use super::*;
use model::Instr;
use model::Val;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub ip: Val,
	pub instrs: Vec <Instr>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { ip, instrs, params } = [ params, "#ip ", ip, "\n", @lines instrs ]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
		pub max_instrs: u32 = ("MAX_INSTRS=", 10_000, 1 .. ),
	}
}
