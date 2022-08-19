use super::*;
use model::Instr;
use model::Sample;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub samples: Vec <Sample>,
	pub instrs: Vec <Instr>,
	pub params: InputParams,
}

impl <'inp> FromParser <'inp> for Input {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, params, @delim "\n\n" samples, "\n\n\n\n", @lines instrs);
		Ok (Self { samples, instrs, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"{params}{samples}\n\n\n\n{instrs}\n",
			params = self.params,
			samples = DisplayDelim::new ("\n\n", & self.samples),
			instrs = DisplayDelim::new ("\n", & self.instrs))
	}
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
	}
}
