use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub input: InpStr <'inp>,
}

impl <'inp> Input <'inp> {
	pub fn parse (input: & [& 'inp str]) -> GenResult <Self> {
		if input.len () != 1 { return Err ("Input must be exactly one line".into ()) }
		Ok (Self { input: InpStr::borrow (input [0]) })
	}
}

impl <'inp> Display for Input <'inp> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "{}\n", self.input) ?;
		Ok (())
	}
}
