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
	pub fn parse (input: & [& str]) -> GenResult <Self> {
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params,
				"Generator A starts with ", start_a, "\n",
				"Generator B starts with ", start_b);
			Ok (Self { start_a, start_b, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter,
			concat! (
				"Generator A starts with {start_a}\n",
				"Generator B starts with {start_b}\n",
			),
			start_a = self.start_a,
			start_b = self.start_b,
		) ?;
		Ok (())
	}
}
