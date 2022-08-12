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
	pub fn parse (mut input: & [& 'inp str]) -> GenResult <Self> {
		let params = InputParams::parse (& mut input) ?;
		if input.len () != 1 { return Err ("Input must be exactly one line".into ()) }
		let key = InpStr::borrow (input [0]);
		Ok (Self { key, params })
	}
}

impl <'inp> Display for Input <'inp> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "{}\n", self.key) ?;
		Ok (())
	}
}
