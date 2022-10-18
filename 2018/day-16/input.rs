use super::*;
use model::Instr;
use model::Sample;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub samples: Vec <Sample>,
	pub instrs: Vec <Instr>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { samples, instrs, params } = [
		params,
		@delim "\n\n" samples,
		"\n\n\n\n",
		@lines instrs,
	]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}
