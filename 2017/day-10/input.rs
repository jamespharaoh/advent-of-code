use super::*;
use parser::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub lengths: InputLengths,
	pub params: InputParams,
}

#[ derive (Clone, Debug) ]
pub struct InputLengths {
	data: Vec <u8>,
}

#[ derive (Clone, Debug) ]
pub struct InputParams {
	pub string_size: u32,
	pub rounds_one: u32,
	pub rounds_two: u32,
}

impl Default for InputParams {
	fn default () -> Self {
		Self {
			string_size: 256,
			rounds_one: 1,
			rounds_two: 64,
		}
	}
}

impl Input {
	pub fn parse (mut input: & [& str]) -> GenResult <Self> {
		let params = InputParams::parse (& mut input) ?;
		let lengths = InputLengths::parse (input) ?;
		Ok (Self { lengths, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		Display::fmt (& self.lengths, formatter) ?;
		Ok (())
	}
}

impl InputLengths {
	pub fn parse (input: & [& str]) -> GenResult <Self> {
		if input.len () != 1 { return Err ("Input must be exactly one line".into ()) }
		let data = Parser::wrap (input [0],
			|parser| {
				let mut items = Vec::new ();
				items.push (parser.uint () ?);
				while ! parser.rest ().is_empty () {
					items.push (parser.expect (",") ?.uint () ?);
				}
				Ok (items)
			}).map_parse_err (|col_idx|
				format! ("Invalid input: col {}: {}", col_idx + 1, input [0])) ?;
		Ok (Self { data })
	}
}

impl Deref for InputLengths {
	type Target = [u8];
	fn deref (& self) -> & [u8] { & self.data }
}

impl Display for InputLengths {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		let mut first = true;
		for length in self.data.iter_vals () {
			if first { first = false; } else { write! (formatter, ",") ?; }
			write! (formatter, "{}", length) ?;
		}
		Ok (())
	}
}

impl InputParams {
	pub fn parse (input: & mut & [& str]) -> GenResult <Self> {
		let default = Self::default ();
		let string_size = parser::input_param (input, "STRING_SIZE=", default.string_size) ?;
		if ! (2 ..= 256).contains (& string_size) {
			return Err ("STRING_SIZE must be between 1 and 256".into ());
		}
		let rounds_one = parser::input_param (input, "ROUNDS_ONE=", default.rounds_one) ?;
		if ! (1 ..= 64).contains (& rounds_one) {
			return Err ("ROUNDS_ONE must be between 1 and 64".into ());
		}
		let rounds_two = parser::input_param (input, "ROUNDS_TWO=", default.rounds_two) ?;
		if ! (1 ..= 64).contains (& rounds_two) {
			return Err ("ROUNDS_TWO must be between 1 and 64".into ());
		}
		Ok (Self { string_size, rounds_one, rounds_two })
	}
}

impl Display for InputParams {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		let default = Self::default ();
		if self.string_size != default.string_size {
			write! (formatter, "STRING_SIZE={}\n", self.string_size) ?;
		}
		if self.rounds_one != default.rounds_one {
			write! (formatter, "ROUNDS_ONE={}\n", self.rounds_one) ?;
		}
		if self.rounds_two != default.rounds_two {
			write! (formatter, "ROUNDS_TWO={}\n", self.rounds_two) ?;
		}
		Ok (())
	}
}
