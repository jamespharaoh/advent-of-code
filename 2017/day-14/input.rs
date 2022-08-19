use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub key: InpStr <'inp>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_rounds: u32 = ("NUM_ROUNDS=", 64, (1_u32 ..= 64)),
		pub num_rows: u32 = ("NUM_ROWS=", 128, (1_u32 ..= 128)),
	}
}

impl <'inp> Input <'inp> {
	pub fn parse (input: & 'inp [& 'inp str]) -> GenResult <Self> {
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params, key);
			Ok (Self { key, params })
		})
	}
}

impl <'inp> Display for Input <'inp> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "{}\n", self.key) ?;
		Ok (())
	}
}
