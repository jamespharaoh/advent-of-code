use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub data: InpStr <'inp>,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub rounds_two: u32 = ("ROUNDS_TWO=", 64, 1 ..= 64),
	}
}

impl <'inp> Input <'inp> {
	pub fn parse (mut input: & [& 'inp str]) -> GenResult <Self> {
		let params = InputParams::parse (& mut input) ?;
		let data = InpStr::borrow (input.first ().copied ().unwrap_or (""));
		Ok (Self { data, params })
	}
}

impl <'inp> Display for Input <'inp> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "{}\n", self.data) ?;
		Ok (())
	}
}
