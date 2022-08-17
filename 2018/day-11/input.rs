use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub serial: i32,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
		pub grid_size: u16 = ("GRID_SIZE=", 300, (2_u16 .. )),
	}
}

impl <'inp> FromParser <'inp> for Input {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, params, serial);
		Ok (Self { serial, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "{}\n", self.serial) ?;
		Ok (())
	}
}
