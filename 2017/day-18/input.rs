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
	pub fn parse (input: & [& str]) -> GenResult <Self> {
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params, @delim "\n" instrs);
			Ok (Self { instrs, params })
		})
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
