use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub polymer: InpStr <'inp>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}

impl <'inp> Input <'inp> {
	pub fn parse (input: & 'inp [& 'inp str]) -> GenResult <Self> {
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params, (@rest polymer = |ch| ch.is_ascii_alphabetic ()));
			Ok (Self { polymer, params })
		})
	}
}

impl <'inp> Display for Input <'inp> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "{}\n", self.polymer) ?;
		Ok (())
	}
}
