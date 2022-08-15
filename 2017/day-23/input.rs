use super::*;
use cpu::Instr;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub instrs: Vec <Instr>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}

impl Input {
	pub fn parse (mut input: & [& str]) -> GenResult <Self> {
		let params = InputParams::parse (& mut input) ?;
		let instrs = Parser::wrap_lines_auto_items (
			input.iter ().copied ().enumerate ()) ?;
		Ok (Self { instrs, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		for instr in self.instrs.iter () {
			write! (formatter, "{}\n", instr) ?;
		}
		Ok (())
	}
}
