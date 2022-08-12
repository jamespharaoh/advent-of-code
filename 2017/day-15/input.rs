use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub start_a: u32,
	pub start_b: u32,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub reps_one: u32 = ("REPS_ONE=", 40_000_000, 0_u32 .. ),
		pub reps_two: u32 = ("REPS_TWO=", 5_000_000, 0_u32 .. ),
	}
}

impl Input {
	pub fn parse (mut input: & [& str]) -> GenResult <Self> {
		let params = InputParams::parse (& mut input) ?;
		if input.len () != 2 { return Err ("Input must be exactly two lines".into ()) }
		let vals_temp = Parser::wrap_lines_auto (
			input.iter ().copied ().enumerate (),
			|parser| {
				let id = parser.expect ("Generator ") ?.expect_next () ?;
				let val = parser.expect (" starts with ") ?.uint () ?;
				parser.end () ?;
				Ok ((id, val))
			}) ?;
		if vals_temp [0].0 != 'A' || vals_temp [1].0 != 'B' {
			return Err ("Invalid input".into ());
		}
		let (start_a, start_b) = (vals_temp [0].1, vals_temp [1].1);
		Ok (Self { start_a, start_b, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "Generator A starts with {}\n", self.start_a) ?;
		write! (formatter, "Generator B starts with {}\n", self.start_b) ?;
		Ok (())
	}
}
