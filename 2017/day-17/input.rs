use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub advance: u32,
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
		if input.len () != 1 { return Err ("Input must be exactly one line".into ()) }
		let advance = Parser::wrap_auto (input [0], |parser| {
			let advance = parser.uint () ?;
			parser.end () ?;
			Ok (advance)
		}) ?;
		Ok (Self { advance, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "{}\n", self.advance) ?;
		Ok (())
	}
}
