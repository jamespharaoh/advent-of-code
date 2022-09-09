use super::*;
use model::Instr;
use model::Val;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub ip: Val,
	pub instrs: Rc <[Instr]>,
	pub params: InputParams,
}

struct_parser_display! (Input { ip, instrs, params } = [
	params, "#ip ", ip, "\n", @lines instrs,
]);

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
		pub max_instrs: u64 = ("MAX_INSTRS=", 5_000_000_000, 1_u64 .. ),
	}
}
